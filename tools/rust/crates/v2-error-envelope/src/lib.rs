//! Structured error envelope shared by v2-* orchestrator primitives.
//!
//! Per `docs/redesign/_notes/v2-structured-error-envelope-arc.md` (C6+C7+C9
//! design landed cycle 181, implementation cycle 1 lands cycle 182).
//!
//! The envelope replaces the substring-matching `classify_failure` path in
//! v2-cycle-runner with a deterministic JSON-on-stderr protocol. Each v2-*
//! primitive supports `--error-format <text|json>`; in JSON mode the failure
//! emits a single line of compact JSON envelope to stderr (last line of
//! stderr is the envelope by convention; pre-envelope warnings allowed).
//!
//! Bottom-up migration (design §6.5):
//! - Cycle 1 (this crate, cycle 182): shared types + emit/parse helpers.
//! - Cycle 2 (cycle 183+): v2-channel-router emit-side.
//! - Cycle 3 (cycle 184+): v2-role-driver emit-side incl OQ-LS-AUTH visibility.
//! - Cycle 4 (cycle 185+): v2-cycle-runner caller-side (classifier rewrite).
//!
//! Schema-promotion-discipline (HARDENED cycle 178): the envelope carries
//! only `{primitive, class, details, human}` at cycle 1. Fields like
//! `invocation_id`, `timestamp`, `process_pid`, `elapsed_ms` are deferred
//! until a concrete reader appears.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use v2_primitive_invoker::TimeoutDiagnostic;

/// Structured error envelope emitted by v2-* primitives on failure paths
/// when invoked with `--error-format=json`. The last non-empty line of the
/// primitive's stderr is the envelope; pre-envelope warning lines are
/// allowed (e.g., lenient-mode validation warnings in v2-channel-router).
///
/// Schema is intentionally minimal (4 fields) per the schema-promotion
/// discipline (cycle 178 `schema-promotion-requires-reader-co-edit-via-named-field`).
/// Forward-compat keys live in `details` until a reader justifies promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    /// Which v2-* primitive emitted this envelope. Kebab-case crate name
    /// (e.g., `"v2-channel-router"`, `"v2-role-driver"`).
    pub primitive: String,

    /// Failure class. See `ErrorClass` for the taxonomy.
    pub class: ErrorClass,

    /// Free-form per-class diagnostic details. Each `ErrorClass` documents
    /// its expected key set (design §4 table) but the shape is intentionally
    /// `Map<String, Value>` rather than a typed enum-per-class so primitives
    /// can extend without coordinating a release across all consumers.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub details: Map<String, Value>,

    /// Human-readable single-line message. The text that would appear on
    /// stderr in `--error-format=text` mode. Used by v2-cycle-runner when
    /// rendering CycleReport in text mode + when JSON parse fails (fallback
    /// to legacy substring classification).
    pub human: String,
}

/// Failure class taxonomy. Extends the v2-cycle-runner internal
/// `FailureClass` enum (6 variants) with 5 new classes (Auth, Config, Io,
/// Protocol, Unknown) per design §3.1. The shared crate is authoritative
/// for the taxonomy; v2-cycle-runner's internal enum will mirror this set
/// at cycle 4 (caller-side migration).
///
/// Variant ordering mirrors the design §4 taxonomy table for readability.
/// Retry-policy: Transient and Unknown retry-once; all others NO retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorClass {
    /// Recoverable; retry-once is safe. Network blip, fs contention, etc.
    Transient,

    /// Authentication / authorization failure. Cycle-180 OQ-LS-AUTH case:
    /// claude-code returns `{is_error: true, result: "Not logged in"}`. The
    /// role-driver maps this to `class=auth, details.subsystem=...` so the
    /// orchestrator observes a typed Auth class rather than the generic
    /// WriteSkipped diagnostic. Expected details: `subsystem` (e.g.,
    /// `"claude-code-oauth"`), optional `upstream_result`.
    Auth,

    /// Configuration failure: missing env var, malformed CLI arg, bad
    /// config file. Expected details: `key` (config name), optional
    /// `expected`.
    Config,

    /// I/O failure: file-not-found, permission-denied, disk-full. Expected
    /// details: `path`, `op` (read/write/stat), optional `errno`.
    Io,

    /// Protocol failure: envelope-parse-failure, sub-protocol mismatch,
    /// schema-mismatch. Expected details: `expected_shape`,
    /// `observed_shape`, `parse_error`. Used by role-driver when
    /// `parse_claude_code_envelope` fails; used by channel-router when
    /// `InvalidPayload` is observed.
    Protocol,

    /// Reducer-rule violation; channel-router refused a write because the
    /// declared writer disagrees with the channel's owning writer. Expected
    /// details: `channel`, `writer`, `reason`. Existing v2-cycle-runner
    /// `FailureClass::ChannelWriteRejected`.
    ChannelWriteRejected,

    /// Role session returned empty output (A4-silent-fail family). Expected
    /// details: `role`, `cycle`. Existing v2-cycle-runner
    /// `FailureClass::RoleSessionEmpty`.
    RoleSessionEmpty,

    /// Super-step sequence violation. Expected details: `current_step`,
    /// `attempted_step`. Existing v2-cycle-runner
    /// `FailureClass::SuperStepOutOfOrder`.
    SuperStepOutOfOrder,

    /// State-bound exceeded by v2-state-audit (hard severity). Expected
    /// details: `dimension`, `observed`, `bound`. Existing v2-cycle-runner
    /// `FailureClass::StateBoundExceeded`.
    StateBoundExceeded,

    /// Per-step timeout; invoker-mediated. Expected details mirror cycle-177
    /// `TimeoutDiagnostic`: `budget_ms`, `elapsed_ms`, `escalation`. Use
    /// `ErrorEnvelope::for_timeout` to construct from a `TimeoutDiagnostic`.
    Timeout,

    /// Forward-compat catch-all: envelope was parseable but `class` value
    /// isn't recognized by this version of cycle-runner. Retry-once mirrors
    /// `Transient` per design §6 OQ-SEE-6 default. Operator sees the
    /// unknown class in CycleReport and decides downstream.
    Unknown,
}

impl ErrorClass {
    /// Kebab-case slug for log lines and human-readable contexts. Mirrors
    /// the serde rename_all but exposes it for non-serde call sites.
    pub fn as_slug(self) -> &'static str {
        match self {
            ErrorClass::Transient => "transient",
            ErrorClass::Auth => "auth",
            ErrorClass::Config => "config",
            ErrorClass::Io => "io",
            ErrorClass::Protocol => "protocol",
            ErrorClass::ChannelWriteRejected => "channel-write-rejected",
            ErrorClass::RoleSessionEmpty => "role-session-empty",
            ErrorClass::SuperStepOutOfOrder => "super-step-out-of-order",
            ErrorClass::StateBoundExceeded => "state-bound-exceeded",
            ErrorClass::Timeout => "timeout",
            ErrorClass::Unknown => "unknown",
        }
    }

    /// Retry policy per design §4 table. Only `Transient` and `Unknown`
    /// admit retry-once; all classed failures halt.
    pub fn is_retryable(self) -> bool {
        matches!(self, ErrorClass::Transient | ErrorClass::Unknown)
    }
}

impl ErrorEnvelope {
    /// Construct an envelope with no details. Details may be added via
    /// `with_detail` / `with_details_map`.
    pub fn new(
        primitive: impl Into<String>,
        class: ErrorClass,
        human: impl Into<String>,
    ) -> Self {
        Self {
            primitive: primitive.into(),
            class,
            details: Map::new(),
            human: human.into(),
        }
    }

    /// Builder: attach a single detail key. Overwrites if key exists.
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    /// Builder: attach a full details map, replacing any existing entries.
    /// Useful for primitives that build their details map separately
    /// (e.g., translating an io::Error into multiple keys).
    pub fn with_details_map(mut self, details: Map<String, Value>) -> Self {
        self.details = details;
        self
    }

    /// Construct a `Timeout`-class envelope from a `TimeoutDiagnostic`. The
    /// invoker-mediated timeout path produces a `TimeoutDiagnostic`; the
    /// primitive translates it to an envelope before exit.
    ///
    /// Details schema (design §4):
    /// - `budget_ms`: u64 — the timeout budget that was exhausted.
    /// - `elapsed_ms`: u64 — actual elapsed at signal-escalation completion.
    /// - `escalation`: string — `"sigterm-clean"` or `"sigkill-forced"`.
    pub fn for_timeout(
        primitive: impl Into<String>,
        diagnostic: &TimeoutDiagnostic,
    ) -> Self {
        let primitive = primitive.into();
        let human = format!(
            "{primitive}: per-step timeout exhausted at {}ms (budget {}ms; {})",
            diagnostic.elapsed_ms,
            diagnostic.budget_ms,
            match diagnostic.escalation {
                v2_primitive_invoker::SignalEscalation::SigtermClean => "SIGTERM clean",
                v2_primitive_invoker::SignalEscalation::SigkillForced => "SIGKILL forced",
            },
        );
        let escalation_slug = match diagnostic.escalation {
            v2_primitive_invoker::SignalEscalation::SigtermClean => "sigterm-clean",
            v2_primitive_invoker::SignalEscalation::SigkillForced => "sigkill-forced",
        };
        Self::new(primitive, ErrorClass::Timeout, human)
            .with_detail("budget_ms", diagnostic.budget_ms)
            .with_detail("elapsed_ms", diagnostic.elapsed_ms)
            .with_detail("escalation", escalation_slug)
    }

    /// Serialize as a single compact JSON line (no trailing newline). The
    /// caller appends a newline when writing to stderr — this matches the
    /// "envelope is the last non-empty line of stderr" convention without
    /// committing to any specific newline policy.
    pub fn to_json_line(&self) -> String {
        serde_json::to_string(self)
            .expect("ErrorEnvelope serialization is infallible for owned types")
    }

    /// Convenience: delegates to `self.class.is_retryable()`. Lets callers
    /// avoid `envelope.class.is_retryable()` indirection in hot paths.
    pub fn is_retryable_class(&self) -> bool {
        self.class.is_retryable()
    }
}

/// Parse the tail of stderr for an `ErrorEnvelope`. Returns `None` if no
/// non-empty line in stderr deserializes as an envelope. Pre-envelope
/// warning lines (e.g., lenient-mode validation in v2-channel-router) are
/// tolerated — only the LAST non-empty line is consulted.
///
/// v2-cycle-runner uses this in its rewritten `classify_failure` (design
/// §3.3): try envelope-first, fall back to legacy substring matching on
/// parse failure (forward-compat with primitives that haven't migrated).
pub fn parse_from_stderr_tail(stderr: &str) -> Option<ErrorEnvelope> {
    stderr
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .and_then(|line| serde_json::from_str::<ErrorEnvelope>(line).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use v2_primitive_invoker::SignalEscalation;

    fn sample_diagnostic() -> TimeoutDiagnostic {
        TimeoutDiagnostic {
            budget_ms: 30_000,
            elapsed_ms: 30_142,
            escalation: SignalEscalation::SigkillForced,
        }
    }

    #[test]
    fn class_slug_matches_serde_rename() {
        // Sanity: as_slug() must mirror what serde produces for the enum
        // variant. Drift here would be a silent bug; the test pins the
        // mapping.
        for (variant, expected) in [
            (ErrorClass::Transient, "transient"),
            (ErrorClass::Auth, "auth"),
            (ErrorClass::Config, "config"),
            (ErrorClass::Io, "io"),
            (ErrorClass::Protocol, "protocol"),
            (ErrorClass::ChannelWriteRejected, "channel-write-rejected"),
            (ErrorClass::RoleSessionEmpty, "role-session-empty"),
            (ErrorClass::SuperStepOutOfOrder, "super-step-out-of-order"),
            (ErrorClass::StateBoundExceeded, "state-bound-exceeded"),
            (ErrorClass::Timeout, "timeout"),
            (ErrorClass::Unknown, "unknown"),
        ] {
            assert_eq!(variant.as_slug(), expected, "as_slug for {variant:?}");
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(
                json,
                format!("\"{expected}\""),
                "serde for {variant:?}",
            );
            let parsed: ErrorClass = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, variant);
        }
    }

    #[test]
    fn retry_policy_only_transient_and_unknown_retry() {
        assert!(ErrorClass::Transient.is_retryable());
        assert!(ErrorClass::Unknown.is_retryable());
        for c in [
            ErrorClass::Auth,
            ErrorClass::Config,
            ErrorClass::Io,
            ErrorClass::Protocol,
            ErrorClass::ChannelWriteRejected,
            ErrorClass::RoleSessionEmpty,
            ErrorClass::SuperStepOutOfOrder,
            ErrorClass::StateBoundExceeded,
            ErrorClass::Timeout,
        ] {
            assert!(!c.is_retryable(), "{c:?} must not be retryable");
        }
    }

    #[test]
    fn envelope_roundtrip_minimal() {
        let envelope = ErrorEnvelope::new(
            "v2-channel-router",
            ErrorClass::ChannelWriteRejected,
            "channel write rejected: declared writer does not own channel",
        );
        let line = envelope.to_json_line();
        // Verify: details key is skipped when empty per skip_serializing_if.
        assert!(!line.contains("\"details\""), "empty details must be skipped: {line}");
        let parsed: ErrorEnvelope = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn envelope_roundtrip_with_details() {
        let envelope = ErrorEnvelope::new(
            "v2-role-driver",
            ErrorClass::Auth,
            "live-spawn auth failure: claude-code returned Not logged in",
        )
        .with_detail("subsystem", "claude-code-oauth")
        .with_detail("upstream_result", "Not logged in");

        let line = envelope.to_json_line();
        assert!(line.contains("\"details\""), "non-empty details must serialize: {line}");
        assert!(line.contains("\"subsystem\""));
        assert!(line.contains("\"claude-code-oauth\""));

        let parsed: ErrorEnvelope = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, envelope);
        assert_eq!(parsed.class, ErrorClass::Auth);
        assert_eq!(
            parsed.details.get("upstream_result").and_then(|v| v.as_str()),
            Some("Not logged in"),
        );
    }

    #[test]
    fn envelope_with_details_map_overwrites() {
        let mut map = Map::new();
        map.insert("channel".into(), Value::String("findings".into()));
        map.insert("writer".into(), Value::String("planner".into()));
        let envelope = ErrorEnvelope::new(
            "v2-channel-router",
            ErrorClass::ChannelWriteRejected,
            "reducer rule rejected write",
        )
        .with_detail("pre-existing", "should be replaced")
        .with_details_map(map);
        assert_eq!(envelope.details.len(), 2);
        assert!(envelope.details.contains_key("channel"));
        assert!(envelope.details.contains_key("writer"));
        assert!(!envelope.details.contains_key("pre-existing"));
    }

    #[test]
    fn timeout_envelope_from_diagnostic_carries_all_fields() {
        let diagnostic = sample_diagnostic();
        let envelope = ErrorEnvelope::for_timeout("v2-role-driver", &diagnostic);
        assert_eq!(envelope.class, ErrorClass::Timeout);
        assert_eq!(envelope.primitive, "v2-role-driver");
        assert!(!envelope.is_retryable_class());
        assert_eq!(envelope.details["budget_ms"], Value::from(30_000_u64));
        assert_eq!(envelope.details["elapsed_ms"], Value::from(30_142_u64));
        assert_eq!(envelope.details["escalation"], Value::from("sigkill-forced"));
        assert!(envelope.human.contains("30142ms"));
        assert!(envelope.human.contains("SIGKILL forced"));
    }

    #[test]
    fn timeout_envelope_sigterm_clean_branch() {
        let diagnostic = TimeoutDiagnostic {
            budget_ms: 5_000,
            elapsed_ms: 5_010,
            escalation: SignalEscalation::SigtermClean,
        };
        let envelope = ErrorEnvelope::for_timeout("v2-channel-router", &diagnostic);
        assert_eq!(envelope.details["escalation"], Value::from("sigterm-clean"));
        assert!(envelope.human.contains("SIGTERM clean"));
    }

    #[test]
    fn timeout_envelope_roundtrips_through_json() {
        let diagnostic = sample_diagnostic();
        let envelope = ErrorEnvelope::for_timeout("v2-role-driver", &diagnostic);
        let line = envelope.to_json_line();
        let parsed: ErrorEnvelope = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn parse_from_stderr_tail_finds_last_line() {
        let envelope = ErrorEnvelope::new(
            "v2-channel-router",
            ErrorClass::Io,
            "missing payload file",
        )
        .with_detail("path", "/tmp/missing.json")
        .with_detail("op", "read");

        let stderr = format!("warning: lenient mode skipped channel-x\n{}", envelope.to_json_line());
        let parsed = parse_from_stderr_tail(&stderr).expect("envelope found");
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn parse_from_stderr_tail_tolerates_trailing_whitespace() {
        let envelope = ErrorEnvelope::new(
            "v2-role-driver",
            ErrorClass::Protocol,
            "envelope parse failure",
        );
        // Trailing blank lines and whitespace must not defeat the parse.
        let stderr = format!("{}\n\n   \n", envelope.to_json_line());
        let parsed = parse_from_stderr_tail(&stderr).expect("envelope found despite trailing whitespace");
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn parse_from_stderr_tail_returns_none_on_non_envelope() {
        let stderr = "v2-channel-router: legacy text-mode error\n";
        assert!(parse_from_stderr_tail(stderr).is_none());
    }

    #[test]
    fn parse_from_stderr_tail_returns_none_on_empty() {
        assert!(parse_from_stderr_tail("").is_none());
        assert!(parse_from_stderr_tail("\n\n   \n").is_none());
    }

    #[test]
    fn parse_from_stderr_tail_ignores_envelope_not_on_last_line() {
        // If a primitive emits the envelope but then writes more text after
        // it (shouldn't happen by spec), the last line is what we honor —
        // the contract is "envelope is last line." This pins that contract.
        let envelope = ErrorEnvelope::new(
            "v2-role-driver",
            ErrorClass::Auth,
            "auth fail",
        );
        let stderr = format!("{}\ntrailing log line not an envelope", envelope.to_json_line());
        assert!(parse_from_stderr_tail(&stderr).is_none());
    }

    #[test]
    fn unknown_class_envelope_roundtrips() {
        // Forward-compat: if a future primitive emits a class we don't know
        // about, serde Deserialize would fail on unknown enum variant. Pin
        // the current behavior — Unknown is a real variant that callers
        // emit explicitly when they themselves don't know the class.
        let envelope = ErrorEnvelope::new(
            "v2-future-primitive",
            ErrorClass::Unknown,
            "something happened",
        )
        .with_detail("note", "primitive-emitted unknown");
        let line = envelope.to_json_line();
        let parsed: ErrorEnvelope = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, envelope);
        assert!(parsed.class.is_retryable());
    }
}
