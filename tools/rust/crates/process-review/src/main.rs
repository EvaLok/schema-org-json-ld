use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, set_value_at_pointer,
    write_state_value, DeferredFinding, FindingDisposition, StateJson,
};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_CATEGORY_LENGTH: usize = 40;
const DEFERRAL_DEADLINE_CYCLES: u64 = 5;
const DROPPED_DEFERRAL_RESOLVED_REF_MAX_CHARS: usize = 100;
const VALID_FINDING_DISPOSITIONS: &[&str] = &[
    "actioned",
    "deferred",
    "dispatch_created",
    "actioned_failed",
    "verified_resolved",
    "ignored",
];
const BUILTIN_KNOWN_CATEGORIES: &[&str] = &[
    "complacency-audit",
    "data-integrity",
    "process-integrity",
    "publish-gate-evidence-reuse",
    "reactive-manual-repair",
    "review-accounting",
    "review-agent-freshness-drift",
    "review-evidence",
    "state-integrity",
    "tooling-contract",
    "worklog-accuracy",
];

#[derive(Parser, Debug)]
#[command(name = "process-review")]
struct Cli {
    /// Optional path to the review file (e.g. docs/reviews/cycle-162.md); may
    /// be omitted when running only --update-chronic-category refreshes
    #[arg(long)]
    review_file: Option<PathBuf>,

    /// Review dispatch issue number for the review file being processed
    #[arg(long)]
    review_issue: Option<u64>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Count of findings actioned this cycle
    #[arg(long, default_value_t = 0)]
    actioned: u64,

    /// Count of findings deferred this cycle
    #[arg(long, default_value_t = 0)]
    deferred: u64,

    /// Count of findings with a dispatch created this cycle
    #[arg(long, default_value_t = 0)]
    dispatch_created: u64,

    /// Count of findings previously actioned but still failing
    #[arg(long, default_value_t = 0)]
    actioned_failed: u64,

    /// Count of findings verified resolved via regression check
    #[arg(long, default_value_t = 0)]
    verified_resolved: u64,

    /// Count of findings ignored this cycle
    #[arg(long, default_value_t = 0)]
    ignored: u64,

    /// Per-finding disposition in CATEGORY:DISPOSITION form; may be repeated once per finding
    #[arg(long = "disposition")]
    finding_dispositions: Vec<String>,

    /// Warn on malformed review artifacts instead of failing before state updates
    #[arg(long, default_value_t = false)]
    lenient: bool,

    /// Optional note for the review history entry
    #[arg(long)]
    note: Option<String>,

    /// Drop an active deferred finding in CATEGORY:DEFERRED_CYCLE:RATIONALE form
    #[arg(long = "drop-deferral")]
    drop_deferrals: Vec<String>,

    /// Resolve an active deferred finding in CATEGORY:DEFERRED_CYCLE:RESOLVED_REF form
    #[arg(long = "resolve-deferral")]
    resolve_deferrals: Vec<String>,

    /// Chronic category id(s) to refresh after a structural fix lands
    #[arg(long = "update-chronic-category")]
    update_chronic_categories: Vec<String>,

    /// Pull request number(s) that addressed the chronic category
    #[arg(long = "update-chronic-pr")]
    update_chronic_prs: Vec<u64>,

    /// Cycle the chronic refresh landed in
    #[arg(long = "update-chronic-cycle")]
    update_chronic_cycle: Option<u64>,

    /// Optional human-readable rationale appended to refreshed chronic entries
    #[arg(long = "update-chronic-rationale")]
    update_chronic_rationale: Option<String>,

    /// Roll back chronic category refreshes in CATEGORY:PRIOR_VC:RATIONALE form
    #[arg(long = "rollback-chronic-category")]
    rollback_chronic_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ReviewHistoryEntry {
    cycle: u64,
    finding_count: u64,
    complacency_score: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    review_issue: Option<u64>,
    categories: Vec<String>,
    actioned: u64,
    deferred: u64,
    #[serde(skip_serializing_if = "is_zero")]
    dispatch_created: u64,
    #[serde(skip_serializing_if = "is_zero")]
    actioned_failed: u64,
    #[serde(skip_serializing_if = "is_zero")]
    verified_resolved: u64,
    ignored: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    finding_dispositions: Vec<FindingDisposition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedReview {
    cycle: u64,
    finding_count: u64,
    complacency_score: u64,
    categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatchUpdate {
    path: String,
    value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReviewRunSummary {
    cycle: u64,
    complacency_score: u64,
    finding_count: u64,
    categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChronicUpdateSummary {
    updated_entries: usize,
    categories: Vec<String>,
}

fn is_zero(value: &u64) -> bool {
    *value == 0
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let has_review_processing = cli.review_file.is_some();
    let has_deferral_update = !cli.drop_deferrals.is_empty() || !cli.resolve_deferrals.is_empty();
    let has_chronic_update = !cli.update_chronic_categories.is_empty();
    let has_chronic_rollback = !cli.rollback_chronic_categories.is_empty();
    if !has_review_processing
        && !has_deferral_update
        && !has_chronic_update
        && !has_chronic_rollback
    {
        return Err("either --review-file, --drop-deferral, --resolve-deferral, --update-chronic-category, or --rollback-chronic-category must be provided".to_string());
    }

    let mut state_value = read_state_value(&cli.repo_root)?;
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;
    let original_state_value = state_value.clone();

    let review_summary = if let Some(review_file) = cli.review_file.as_ref() {
        let review_path = resolve_review_path(&cli.repo_root, review_file);
        let review_content = fs::read_to_string(&review_path)
            .map_err(|error| format!("failed to read {}: {}", review_path.display(), error))?;

        let parsed_review = parse_review(&review_path, &review_content, cli.lenient)?;
        let finding_dispositions = validate_dispositions(&cli, &parsed_review)?;
        let review_issue = cli.review_issue.ok_or_else(|| {
            format!(
                "--review-issue is required when processing review file {}",
                review_path.display()
            )
        })?;
        for warning in validate_categories(&cli.repo_root, &parsed_review.categories)? {
            eprintln!("{}", warning);
        }
        let entry = build_history_entry(&parsed_review, &cli, review_issue, finding_dispositions);

        let (patch, warnings) = build_state_patch(
            &state_value,
            parsed_review.cycle,
            current_cycle,
            &entry,
            &cli.drop_deferrals,
            &cli.resolve_deferrals,
        )?;
        for warning in warnings {
            eprintln!("{}", warning);
        }
        apply_patch(&mut state_value, &patch)?;

        Some(ReviewRunSummary {
            cycle: parsed_review.cycle,
            complacency_score: parsed_review.complacency_score,
            finding_count: parsed_review.finding_count,
            categories: parsed_review.categories,
        })
    } else {
        None
    };

    if !has_review_processing && has_deferral_update {
        let (patch, warnings) = deferred_findings_patch(
            &state_value,
            None,
            None,
            &cli.drop_deferrals,
            &cli.resolve_deferrals,
        )?;
        for warning in warnings {
            eprintln!("{}", warning);
        }
        if let Some(patch) = patch {
            apply_patch(&mut state_value, &[patch])?;
        }
    }

    let chronic_update_summary = if has_chronic_update {
        Some(update_chronic_category_responses(
            &mut state_value,
            &cli.update_chronic_categories,
            &cli.update_chronic_prs,
            cli.update_chronic_cycle.unwrap_or(current_cycle),
            cli.update_chronic_rationale.as_deref(),
        )?)
    } else {
        None
    };
    let chronic_rollback_summary = if has_chronic_rollback {
        Some(rollback_chronic_category_responses(
            &mut state_value,
            &cli.rollback_chronic_categories,
            current_cycle,
        )?)
    } else {
        None
    };

    let receipt = if state_value != original_state_value {
        write_state_value(&cli.repo_root, &state_value)?;
        let commit_message = build_commit_message(
            review_summary.as_ref(),
            chronic_update_summary.as_ref(),
            chronic_rollback_summary.as_ref(),
            has_deferral_update,
            current_cycle,
        );
        Some(commit_state_json(&cli.repo_root, &commit_message)?)
    } else {
        None
    };

    if let Some(review_summary) = review_summary.as_ref() {
        println!(
            "Review processed: cycle {}, score {}/5, {} findings",
            review_summary.cycle, review_summary.complacency_score, review_summary.finding_count
        );
        if review_summary.categories.is_empty() {
            println!("Categories: (none parsed)");
        } else {
            println!("Categories: {}", review_summary.categories.join(", "));
        }
    }
    if let Some(chronic_update_summary) = chronic_update_summary.as_ref() {
        println!(
            "Updated {} chronic_category_responses entries for categories: {}",
            chronic_update_summary.updated_entries,
            chronic_update_summary.categories.join(", ")
        );
    }
    if let Some(chronic_rollback_summary) = chronic_rollback_summary.as_ref() {
        println!(
            "Rolled back {} chronic_category_responses entries for categories: {}",
            chronic_rollback_summary.updated_entries,
            chronic_rollback_summary.categories.join(", ")
        );
    }
    match (review_summary.is_some(), receipt.as_deref()) {
        (true, Some(receipt)) => println!("History entry added (receipt: {})", receipt),
        (false, Some(receipt)) => println!("State updated (receipt: {})", receipt),
        (_, None) => println!("No state changes needed"),
    }

    Ok(())
}

fn build_commit_message(
    review_summary: Option<&ReviewRunSummary>,
    chronic_update_summary: Option<&ChronicUpdateSummary>,
    chronic_rollback_summary: Option<&ChronicUpdateSummary>,
    has_deferral_update: bool,
    current_cycle: u64,
) -> String {
    if review_summary.is_none()
        && chronic_update_summary.is_none()
        && chronic_rollback_summary.is_none()
        && has_deferral_update
    {
        return format!(
            "state(process-review): updated deferred findings [cycle {}]",
            current_cycle
        );
    }
    if review_summary.is_none()
        && chronic_update_summary.is_none()
        && chronic_rollback_summary.is_none()
        && !has_deferral_update
    {
        return format!("state(process-review): no-op [cycle {}]", current_cycle);
    }

    let mut details = Vec::new();
    if let Some(review_summary) = review_summary {
        if chronic_update_summary.is_none()
            && chronic_rollback_summary.is_none()
            && !has_deferral_update
        {
            details.push(format!("score {}/5", review_summary.complacency_score));
        }
    }
    if let Some(chronic_update_summary) = chronic_update_summary {
        details.push(format!(
            "refreshed chronic categories {}",
            chronic_update_summary.categories.join(", ")
        ));
    }
    if let Some(chronic_rollback_summary) = chronic_rollback_summary {
        details.push(format!(
            "rolled back chronic categories {}",
            chronic_rollback_summary.categories.join(", ")
        ));
    }
    if has_deferral_update {
        details.push("updated deferred findings".to_string());
    }

    let prefix = match review_summary {
        Some(review_summary) => format!(
            "state(process-review): cycle {} review consumed",
            review_summary.cycle
        ),
        None if chronic_update_summary.is_none()
            && chronic_rollback_summary.is_some()
            && !has_deferral_update =>
        {
            "state(process-review): chronic rollback".to_string()
        }
        None => "state(process-review): state update".to_string(),
    };

    format!(
        "{}: {} [cycle {}]",
        prefix,
        details.join(" + "),
        current_cycle
    )
}

fn validate_dispositions(
    cli: &Cli,
    parsed_review: &ParsedReview,
) -> Result<Vec<FindingDisposition>, String> {
    let disposition_sum = cli
        .actioned
        .checked_add(cli.deferred)
        .and_then(|value| value.checked_add(cli.dispatch_created))
        .and_then(|value| value.checked_add(cli.actioned_failed))
        .and_then(|value| value.checked_add(cli.verified_resolved))
        .and_then(|value| value.checked_add(cli.ignored))
        .ok_or_else(|| "disposition counts overflowed u64".to_string())?;

    if disposition_sum != parsed_review.finding_count {
        return Err(format!(
            "disposition counts must sum to the parsed finding count: expected {} from --actioned + --deferred + --dispatch-created + --actioned-failed + --verified-resolved + --ignored, got {}",
            parsed_review.finding_count,
            disposition_sum
        ));
    }

    if cli.finding_dispositions.is_empty() {
        return Ok(Vec::new());
    }

    let expected_len = usize::try_from(parsed_review.finding_count)
        .map_err(|error| format!("finding count exceeds usize: {}", error))?;
    if cli.finding_dispositions.len() != expected_len {
        return Err(format!(
            "--disposition must be provided once per finding when used: expected {} entries, got {}",
            parsed_review.finding_count,
            cli.finding_dispositions.len()
        ));
    }

    let known_categories = parsed_review
        .categories
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut parsed_dispositions = Vec::with_capacity(cli.finding_dispositions.len());
    for raw in &cli.finding_dispositions {
        let (category, disposition) = parse_finding_disposition(raw)?;
        if !known_categories.contains(&category) {
            return Err(format!(
                "--disposition category '{}' does not appear in the review file categories",
                category
            ));
        }
        parsed_dispositions.push(FindingDisposition {
            category,
            disposition,
        });
    }

    let counts = count_finding_dispositions(&parsed_dispositions)?;
    if counts.actioned != cli.actioned
        || counts.deferred != cli.deferred
        || counts.dispatch_created != cli.dispatch_created
        || counts.actioned_failed != cli.actioned_failed
        || counts.verified_resolved != cli.verified_resolved
        || counts.ignored != cli.ignored
    {
        return Err(format!(
            "--disposition aggregate counts do not match the provided totals: actioned={}, deferred={}, dispatch_created={}, actioned_failed={}, verified_resolved={}, ignored={}",
            counts.actioned,
            counts.deferred,
            counts.dispatch_created,
            counts.actioned_failed,
            counts.verified_resolved,
            counts.ignored
        ));
    }

    Ok(parsed_dispositions)
}

fn parse_finding_disposition(raw: &str) -> Result<(String, String), String> {
    let (category_raw, disposition_raw) = raw.split_once(':').ok_or_else(|| {
        format!(
            "invalid --disposition '{}': expected CATEGORY:DISPOSITION",
            raw
        )
    })?;
    let category = normalize_category(category_raw).ok_or_else(|| {
        format!(
            "invalid --disposition category '{}': category must normalize to a non-empty slug",
            category_raw
        )
    })?;
    let disposition = disposition_raw.trim();
    if !VALID_FINDING_DISPOSITIONS.contains(&disposition) {
        return Err(format!(
            "invalid --disposition value '{}': expected one of {}",
            disposition,
            VALID_FINDING_DISPOSITIONS.join(", ")
        ));
    }

    Ok((category, disposition.to_string()))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct DispositionCounts {
    actioned: u64,
    deferred: u64,
    dispatch_created: u64,
    actioned_failed: u64,
    verified_resolved: u64,
    ignored: u64,
}

fn count_finding_dispositions(
    dispositions: &[FindingDisposition],
) -> Result<DispositionCounts, String> {
    let mut counts = DispositionCounts::default();
    for disposition in dispositions {
        match disposition.disposition.as_str() {
            "actioned" => counts.actioned += 1,
            "deferred" => counts.deferred += 1,
            "dispatch_created" => counts.dispatch_created += 1,
            "actioned_failed" => counts.actioned_failed += 1,
            "verified_resolved" => counts.verified_resolved += 1,
            "ignored" => counts.ignored += 1,
            other => {
                return Err(format!(
                    "invalid validated disposition encountered while counting: {}",
                    other
                ));
            }
        }
    }

    Ok(counts)
}

fn validate_categories(repo_root: &Path, categories: &[String]) -> Result<Vec<String>, String> {
    let known_categories = known_review_categories(repo_root)?;
    Ok(categories
        .iter()
        .filter(|category| !known_categories.contains(*category))
        .map(|category| format!("WARNING: unrecognized review category: {}", category))
        .collect())
}

fn known_review_categories(repo_root: &Path) -> Result<BTreeSet<String>, String> {
    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse docs/state.json: {}", error))?;
    known_review_categories_from_state(&state)
}

fn known_review_categories_from_state(state: &StateJson) -> Result<BTreeSet<String>, String> {
    let review_agent = state.review_agent()?;
    let mut categories = BUILTIN_KNOWN_CATEGORIES
        .iter()
        .map(|category| (*category).to_string())
        .collect::<BTreeSet<_>>();

    for category in review_agent
        .history
        .iter()
        .flat_map(|entry| entry.categories.iter())
    {
        if let Some(normalized) = normalize_category(category) {
            categories.insert(normalized);
        }
    }

    if let Some(chronic_category_responses) = review_agent.chronic_category_responses.as_ref() {
        categories.extend(chronic_response_categories(chronic_category_responses));
    }

    Ok(categories)
}

fn chronic_response_categories(value: &Value) -> BTreeSet<String> {
    value
        .get("entries")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("category").and_then(Value::as_str))
        .filter_map(normalize_category)
        .collect()
}

fn resolve_review_path(repo_root: &Path, review_file: &Path) -> PathBuf {
    if review_file.is_absolute() {
        review_file.to_path_buf()
    } else {
        repo_root.join(review_file)
    }
}

fn parse_review(review_path: &Path, content: &str, lenient: bool) -> Result<ParsedReview, String> {
    let cycle = extract_cycle_number(review_path).ok_or_else(|| {
        format!(
            "failed to derive cycle number from {}",
            review_path.display()
        )
    })?;

    let validation_errors = validate_review_format(content);
    if !validation_errors.is_empty() {
        if lenient {
            for error in &validation_errors {
                eprintln!(
                    "Warning: {} (continuing because --lenient was passed)",
                    error
                );
            }
        } else {
            return Err(validation_errors.join("; "));
        }
    }

    let complacency_score = match extract_score(content) {
        Some(value) => value,
        None => {
            eprintln!("Warning: unable to parse complacency score from review markdown");
            return Err("complacency score is required".to_string());
        }
    };

    let finding_count = match extract_finding_count(content) {
        Some(value) => value,
        None => {
            eprintln!("Warning: unable to parse finding count from review markdown");
            return Err("finding count is required".to_string());
        }
    };

    let categories = extract_categories(content);
    if categories.is_empty() {
        eprintln!("Warning: no finding categories were parsed from review markdown");
    }

    Ok(ParsedReview {
        cycle,
        finding_count,
        complacency_score,
        categories,
    })
}

fn validate_review_format(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut errors = Vec::new();
    let mut found_heading = false;
    for_each_finding_line(&lines, |index, trimmed| {
        if let Some(number) = numbered_finding_number(trimmed) {
            found_heading = true;

            match resolve_finding_category(trimmed, next_non_empty_line(&lines, index + 1)) {
                Some(raw_category) => {
                    if normalize_category(raw_category).is_none() {
                        errors.push(format!("Finding {} has an invalid category tag", number));
                    }
                }
                None => errors.push(format!("Finding {} has no category tag", number)),
            }
        }
    });

    if !found_heading {
        errors.push("review markdown must contain at least one numbered finding heading in the Findings section".to_string());
    }

    if extract_score(content).is_none() {
        errors.push(
            "review markdown must contain a complacency score section with a parsable N/5 score"
                .to_string(),
        );
    }

    errors
}

fn extract_cycle_number(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?.to_ascii_lowercase();
    if !name.ends_with(".md") {
        return None;
    }

    let stem = name.trim_end_matches(".md");
    if let Some(number) = stem.strip_prefix("cycle-") {
        return number.parse::<u64>().ok();
    }

    None
}

fn extract_score(content: &str) -> Option<u64> {
    let mut in_complacency = false;

    for line in content.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();
        let is_complacency_heading = lower.starts_with("## complacency");
        let is_level_two_heading = lower.starts_with("## ");

        if is_complacency_heading {
            in_complacency = true;
            continue;
        }

        if in_complacency && is_level_two_heading {
            break;
        }

        if !in_complacency || !trimmed.contains("/5") {
            continue;
        }

        if let Some((score, _)) = find_number_before_token(trimmed, "/5") {
            return Some(score);
        }
    }

    None
}

fn extract_finding_count(content: &str) -> Option<u64> {
    let list_count = count_numbered_findings(content);
    if list_count > 0 {
        return Some(list_count as u64);
    }

    None
}

fn count_numbered_findings(content: &str) -> usize {
    let lines: Vec<&str> = content.lines().collect();
    let mut count = 0usize;

    for_each_finding_line(&lines, |_, trimmed| {
        if is_numbered_finding_heading(trimmed) {
            count += 1;
        }
    });

    count
}

fn is_numbered_finding_heading(line: &str) -> bool {
    numbered_finding_number(line).is_some()
}

fn numbered_finding_number(line: &str) -> Option<u64> {
    let mut chars = strip_hash_heading_prefix(line).chars().peekable();
    let mut digits = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            digits.push(*ch);
            chars.next();
        } else {
            break;
        }
    }

    if digits.is_empty() {
        return None;
    }

    if !matches!(chars.next(), Some('.')) {
        return None;
    }

    let mut saw_whitespace = false;
    while let Some(ch) = chars.peek() {
        if ch.is_whitespace() {
            saw_whitespace = true;
            chars.next();
        } else {
            break;
        }
    }

    if !saw_whitespace {
        return None;
    }

    let remainder: String = chars.collect();
    let remainder = remainder.trim_start();

    if !(remainder.starts_with("**") || remainder.starts_with('[')) {
        return None;
    }

    digits.parse::<u64>().ok()
}

fn extract_categories(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut categories = BTreeSet::new();
    for_each_finding_line(&lines, |index, trimmed| {
        if is_numbered_finding_heading(trimmed) {
            if let Some(raw) =
                resolve_finding_category(trimmed, next_non_empty_line(&lines, index + 1))
            {
                if let Some(normalized) = normalize_category(raw) {
                    categories.insert(normalized);
                }
            }
        }
    });

    categories.into_iter().collect()
}

fn for_each_finding_line(lines: &[&str], mut visitor: impl FnMut(usize, &str)) {
    match findings_line_bounds(lines) {
        Some((start, end)) => for_each_non_code_line(&lines[start..end], start, &mut visitor),
        None => for_each_non_code_line(lines, 0, &mut visitor),
    }
}

fn findings_line_bounds(lines: &[&str]) -> Option<(usize, usize)> {
    let start = lines
        .iter()
        .position(|line| line.trim().to_ascii_lowercase().starts_with("## findings"))?;

    for (offset, line) in lines[start + 1..].iter().enumerate() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();
        if lower.starts_with("## ")
            && !lower.starts_with("## findings")
            && !matches_numbered_finding_with_hash_prefix(trimmed)
        {
            return Some((start + 1, start + 1 + offset));
        }
    }

    Some((start + 1, lines.len()))
}

fn for_each_non_code_line(
    lines: &[&str],
    start_index: usize,
    visitor: &mut impl FnMut(usize, &str),
) {
    let mut in_code_block = false;
    for (offset, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            continue;
        }

        visitor(start_index + offset, trimmed);
    }
}

fn extract_inline_category(line: &str) -> Option<&str> {
    let remainder = strip_hash_heading_prefix(line);
    let (_, remainder) = remainder
        .split_once("**[")
        .or_else(|| remainder.split_once('['))?;
    let (category, _) = remainder.split_once(']')?;
    if category.is_empty() {
        None
    } else {
        Some(category)
    }
}

fn extract_category_line(line: &str) -> Option<&str> {
    let raw = line.trim_start().strip_prefix("Category:")?.trim();
    if raw.is_empty() {
        None
    } else {
        Some(raw)
    }
}

fn next_non_empty_line<'a>(lines: &[&'a str], start_index: usize) -> Option<&'a str> {
    if start_index >= lines.len() {
        return None;
    }

    lines[start_index..]
        .iter()
        .copied()
        .find(|line| !line.trim().is_empty())
}

fn resolve_finding_category<'a>(
    heading_line: &'a str,
    next_non_empty_line: Option<&'a str>,
) -> Option<&'a str> {
    extract_inline_category(heading_line)
        .or_else(|| next_non_empty_line.and_then(extract_category_line))
}

fn normalize_category(category: &str) -> Option<String> {
    let mut normalized = String::new();
    let mut last_dash = false;

    for ch in category.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            normalized.push('-');
            last_dash = true;
        }
    }

    let trimmed = normalized.trim_matches('-').to_string();
    if trimmed.is_empty() || trimmed.len() > MAX_CATEGORY_LENGTH {
        None
    } else {
        Some(trimmed)
    }
}

fn strip_hash_heading_prefix(line: &str) -> &str {
    match line.strip_prefix("## ") {
        Some(remainder)
            if remainder
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_digit()) =>
        {
            remainder
        }
        _ => line,
    }
}

fn matches_numbered_finding_with_hash_prefix(line: &str) -> bool {
    matches!(
        line.strip_prefix("## "),
        Some(remainder) if remainder
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
            && numbered_finding_number(remainder).is_some()
    )
}

fn find_number_before_token(text: &str, token: &str) -> Option<(u64, usize)> {
    let token_index = text.find(token)?;
    let mut end = token_index;

    while end > 0 && text.as_bytes()[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    let mut start = end;
    while start > 0 && text.as_bytes()[start - 1].is_ascii_digit() {
        start -= 1;
    }

    if start == end {
        return None;
    }

    let value = text[start..end].parse::<u64>().ok()?;
    Some((value, start))
}

fn build_history_entry(
    parsed_review: &ParsedReview,
    cli: &Cli,
    review_issue: u64,
    finding_dispositions: Vec<FindingDisposition>,
) -> ReviewHistoryEntry {
    ReviewHistoryEntry {
        cycle: parsed_review.cycle,
        finding_count: parsed_review.finding_count,
        complacency_score: parsed_review.complacency_score,
        review_issue: Some(review_issue),
        categories: parsed_review.categories.clone(),
        actioned: cli.actioned,
        deferred: cli.deferred,
        dispatch_created: cli.dispatch_created,
        actioned_failed: cli.actioned_failed,
        verified_resolved: cli.verified_resolved,
        ignored: cli.ignored,
        note: cli.note.clone(),
        finding_dispositions,
    }
}

fn build_state_patch(
    state: &Value,
    review_cycle: u64,
    current_cycle: u64,
    entry: &ReviewHistoryEntry,
    drop_deferrals: &[String],
    resolve_deferrals: &[String],
) -> Result<(Vec<PatchUpdate>, Vec<String>), String> {
    let history = state
        .pointer("/review_agent/history")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing /review_agent/history array in docs/state.json".to_string())?;

    let mut next_history = history.clone();
    let entry_value = serde_json::to_value(entry)
        .map_err(|error| format!("failed to serialize review history entry: {}", error))?;

    let existing_index = next_history
        .iter()
        .position(|item| item.get("cycle").and_then(Value::as_u64) == Some(entry.cycle));

    let (deferred_findings_patch, mut warnings) = deferred_findings_patch(
        state,
        Some(review_cycle),
        Some(entry),
        drop_deferrals,
        resolve_deferrals,
    )?;

    if let Some(index) = existing_index {
        next_history[index] = entry_value;
        warnings.push(format!(
            "warning: cycle {} history entry already existed; replaced with updated dispositions",
            entry.cycle
        ));
    } else {
        next_history.push(entry_value);
    }

    let mut patch = vec![
        PatchUpdate {
            path: "/review_agent/last_review_cycle".to_string(),
            value: json!(review_cycle),
        },
        PatchUpdate {
            path: "/review_agent/history".to_string(),
            value: Value::Array(next_history),
        },
    ];
    if let Some(deferred_findings_patch) = deferred_findings_patch {
        patch.push(deferred_findings_patch);
    }
    patch.push(PatchUpdate {
        path: "/field_inventory/fields/review_agent/last_refreshed".to_string(),
        value: json!(format!("cycle {}", current_cycle)),
    });

    Ok((patch, warnings))
}

fn apply_patch(state: &mut Value, updates: &[PatchUpdate]) -> Result<(), String> {
    for update in updates {
        match set_value_at_pointer(state, &update.path, update.value.clone()) {
            Ok(_) => continue,
            Err(_) if insert_missing_top_level_path(state, &update.path, update.value.clone()) => {
                continue;
            }
            Err(error) => return Err(error),
        }
    }

    Ok(())
}

fn update_chronic_category_responses(
    state: &mut Value,
    categories: &[String],
    prs: &[u64],
    cycle: u64,
    rationale_text: Option<&str>,
) -> Result<ChronicUpdateSummary, String> {
    let requested_categories = categories.iter().cloned().collect::<BTreeSet<_>>();
    let requested_category_list = requested_categories.iter().cloned().collect::<Vec<_>>();
    let entries = state
        .pointer_mut("/review_agent/chronic_category_responses/entries")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            "missing /review_agent/chronic_category_responses/entries array in docs/state.json"
                .to_string()
        })?;

    let rationale_suffix = chronic_refresh_rationale(cycle, prs, rationale_text);
    let idempotence_marker = format!("Cycle {}: refreshed via PR(s)", cycle);
    let mut updated_entries = 0usize;
    let mut matched_categories = BTreeSet::new();

    for (entry_index, entry) in entries.iter_mut().enumerate() {
        let entry_object = entry.as_object_mut().ok_or_else(|| {
            format!(
                "review_agent.chronic_category_responses.entries[{}] must be an object",
                entry_index
            )
        })?;
        let Some(category) = entry_object.get("category").and_then(Value::as_str) else {
            continue;
        };
        if !requested_categories.contains(category) {
            continue;
        }

        matched_categories.insert(category.to_string());
        updated_entries += 1;
        entry_object.insert("updated_cycle".to_string(), json!(cycle));
        entry_object.insert("verification_cycle".to_string(), json!(cycle));

        let existing_rationale = match entry_object.get("rationale") {
            Some(Value::String(value)) => Some(value.clone()),
            Some(Value::Null) | None => None,
            Some(_) => {
                return Err(format!(
                    "review_agent.chronic_category_responses.entries[{}].rationale must be a string when present",
                    entry_index
                ))
            }
        };
        if existing_rationale
            .as_deref()
            .map(|value| value.contains(&idempotence_marker))
            .unwrap_or(false)
        {
            continue;
        }

        let next_rationale = match existing_rationale
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            Some(existing) => format!("{} | {}", existing, rationale_suffix),
            None => rationale_suffix.clone(),
        };
        entry_object.insert("rationale".to_string(), Value::String(next_rationale));
    }

    let missing_categories = requested_categories
        .difference(&matched_categories)
        .cloned()
        .collect::<Vec<_>>();
    if !missing_categories.is_empty() {
        return Err(format!(
            "no chronic_category_responses entries found for categories: {}",
            missing_categories.join(", ")
        ));
    }

    set_value_at_pointer(
        state,
        "/field_inventory/fields/review_agent.chronic_category_responses/last_refreshed",
        json!(format!("cycle {}", cycle)),
    )?;

    Ok(ChronicUpdateSummary {
        updated_entries,
        categories: requested_category_list,
    })
}

fn rollback_chronic_category_responses(
    state: &mut Value,
    rollbacks: &[String],
    current_cycle: u64,
) -> Result<ChronicUpdateSummary, String> {
    let parsed_rollbacks = rollbacks
        .iter()
        .map(|raw| parse_chronic_rollback(raw))
        .collect::<Result<Vec<_>, _>>()?;
    let requested_categories = parsed_rollbacks
        .iter()
        .map(|(category, _, _)| category.clone())
        .collect::<BTreeSet<_>>();
    let requested_category_list = requested_categories.iter().cloned().collect::<Vec<_>>();
    let entries_snapshot = state
        .pointer("/review_agent/chronic_category_responses/entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            "missing /review_agent/chronic_category_responses/entries array in docs/state.json"
                .to_string()
        })?;
    let mut matched_categories = BTreeSet::new();

    for (category, prior_verification_cycle, _) in &parsed_rollbacks {
        let mut matched = false;
        for (entry_index, entry) in entries_snapshot.iter().enumerate() {
            let entry_object = entry.as_object().ok_or_else(|| {
                format!(
                    "review_agent.chronic_category_responses.entries[{}] must be an object",
                    entry_index
                )
            })?;
            let Some(entry_category) = entry_object.get("category").and_then(Value::as_str) else {
                continue;
            };
            if entry_category != category {
                continue;
            }

            matched = true;
            matched_categories.insert(category.clone());
            let current_verification_cycle = entry_object
				.get("verification_cycle")
				.and_then(Value::as_u64)
				.ok_or_else(|| {
					format!(
						"review_agent.chronic_category_responses.entries[{}].verification_cycle must be a numeric value",
						entry_index
					)
				})?;
            if *prior_verification_cycle >= current_verification_cycle {
                return Err(format!(
					"invalid --rollback-chronic-category prior verification cycle {} for category '{}': must be less than current verification_cycle {}",
					prior_verification_cycle, category, current_verification_cycle
				));
            }
            entry_object
                .get("updated_cycle")
                .and_then(Value::as_u64)
                .ok_or_else(|| {
                    format!(
						"review_agent.chronic_category_responses.entries[{}].updated_cycle must be a numeric value",
						entry_index
					)
                })?;
            match entry_object.get("rationale") {
                Some(Value::String(_)) | Some(Value::Null) | None => {}
                Some(_) => {
                    return Err(format!(
						"review_agent.chronic_category_responses.entries[{}].rationale must be a string when present",
						entry_index
					))
                }
            }
        }
        if !matched {
            return Err(format!(
                "no chronic_category_responses entries found for categories: {}",
                category
            ));
        }
    }

    let entries = state
        .pointer_mut("/review_agent/chronic_category_responses/entries")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            "missing /review_agent/chronic_category_responses/entries array in docs/state.json"
                .to_string()
        })?;
    let mut updated_entries = 0usize;

    for (category, prior_verification_cycle, rationale_text) in parsed_rollbacks {
        for (entry_index, entry) in entries.iter_mut().enumerate() {
            let entry_object = entry.as_object_mut().ok_or_else(|| {
                format!(
                    "review_agent.chronic_category_responses.entries[{}] must be an object",
                    entry_index
                )
            })?;
            let Some(entry_category) = entry_object.get("category").and_then(Value::as_str) else {
                continue;
            };
            if entry_category != category {
                continue;
            }

            let prior_updated_cycle = entry_object
                .get("updated_cycle")
                .and_then(Value::as_u64)
                .expect("updated_cycle should be validated before mutation");
            let existing_rationale = match entry_object.get("rationale") {
                Some(Value::String(value)) => Some(value.clone()),
                Some(Value::Null) | None => None,
                Some(_) => unreachable!("rationale type validated before mutation"),
            };
            let rollback_marker = format!(
                "Cycle {}: rollback of cycle {} refresh — {}",
                current_cycle, prior_updated_cycle, rationale_text
            );
            let next_rationale = match existing_rationale
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                Some(existing) => format!("{} | {}", existing, rollback_marker),
                None => rollback_marker,
            };

            entry_object.insert(
                "verification_cycle".to_string(),
                json!(prior_verification_cycle),
            );
            entry_object.insert("updated_cycle".to_string(), json!(current_cycle));
            entry_object.insert("rationale".to_string(), Value::String(next_rationale));
            updated_entries += 1;
        }
    }

    set_value_at_pointer(
        state,
        "/field_inventory/fields/review_agent.chronic_category_responses/last_refreshed",
        json!(format!("cycle {}", current_cycle)),
    )?;

    Ok(ChronicUpdateSummary {
        updated_entries,
        categories: requested_category_list,
    })
}

fn chronic_refresh_rationale(cycle: u64, prs: &[u64], rationale_text: Option<&str>) -> String {
    let mut rationale = format!("Cycle {}: refreshed via PR(s)", cycle);
    if !prs.is_empty() {
        let refs = prs
            .iter()
            .map(|pr| format!("#{}", pr))
            .collect::<Vec<_>>()
            .join(", ");
        rationale.push(' ');
        rationale.push('[');
        rationale.push_str(&refs);
        rationale.push(']');
    }
    if let Some(text) = rationale_text
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        rationale.push_str(" — ");
        rationale.push_str(text);
    }

    rationale
}

fn deferred_findings_patch(
    state: &Value,
    review_cycle: Option<u64>,
    entry: Option<&ReviewHistoryEntry>,
    drop_deferrals: &[String],
    resolve_deferrals: &[String],
) -> Result<(Option<PatchUpdate>, Vec<String>), String> {
    let mut deferred_findings = match state.get("deferred_findings") {
        Some(Value::Array(_)) => serde_json::from_value::<Vec<DeferredFinding>>(
            state
                .get("deferred_findings")
                .cloned()
                .expect("deferred_findings presence already checked"),
        )
        .map_err(|error| {
            format!(
                "failed to parse deferred_findings from state.json: {}",
                error
            )
        })?,
        Some(_) => return Err("/deferred_findings must be an array when present".to_string()),
        None => Vec::new(),
    };
    let existing_snapshot = deferred_findings.clone();
    let mut warnings = Vec::new();
    if let (Some(review_cycle), Some(entry)) = (review_cycle, entry) {
        let resolved_ref = format!("docs/reviews/cycle-{}.md", review_cycle);

        let resolved_categories = entry
            .finding_dispositions
            .iter()
            .filter(|disposition| {
                matches!(
                    disposition.disposition.as_str(),
                    "actioned" | "dispatch_created" | "verified_resolved"
                )
            })
            .map(|disposition| disposition.category.clone())
            .collect::<BTreeSet<_>>();
        for category in resolved_categories {
            if let Some(existing) = deferred_findings
                .iter_mut()
                .rev()
                .find(|finding| finding.category == category && is_active_deferred_finding(finding))
            {
                existing.resolved = true;
                existing.resolved_ref = Some(resolved_ref.clone());
            }
        }

        let deferred_categories = entry
            .finding_dispositions
            .iter()
            .filter(|disposition| disposition.disposition == "deferred")
            .map(|disposition| disposition.category.clone())
            .collect::<BTreeSet<_>>();
        for category in deferred_categories {
            if deferred_findings
                .iter()
                .any(|finding| finding.category == category && is_active_deferred_finding(finding))
            {
                warnings.push(format!(
                    "warning: category '{}' already has an unresolved deferred finding; keeping the existing entry",
                    category
                ));
                continue;
            }

            deferred_findings.push(DeferredFinding {
                category,
                deferred_cycle: review_cycle,
                deadline_cycle: review_cycle
                    .checked_add(DEFERRAL_DEADLINE_CYCLES)
                    .ok_or_else(|| "review cycle overflowed deadline calculation".to_string())?,
                resolved: false,
                resolved_ref: None,
                dropped_rationale: None,
            });
        }
    }
    for raw in drop_deferrals {
        let (category, deferred_cycle, rationale) =
            parse_deferral_update(raw, "--drop-deferral", "RATIONALE")?;
        let finding = deferred_findings
            .iter_mut()
            .rev()
            .find(|finding| {
                finding.category == category
                    && finding.deferred_cycle == deferred_cycle
                    && is_active_deferred_finding(finding)
            })
            .ok_or_else(|| {
                format!(
                    "no active deferred finding found for category '{}' deferred in cycle {} to drop",
                    category, deferred_cycle
                )
            })?;
        let resolved_ref = format!(
            "dropped: {}",
            rationale
                .chars()
                .take(DROPPED_DEFERRAL_RESOLVED_REF_MAX_CHARS)
                .collect::<String>()
        );
        finding.dropped_rationale = Some(rationale);
        finding.resolved = true;
        finding.resolved_ref = Some(resolved_ref);
    }
    for raw in resolve_deferrals {
        let (category, deferred_cycle, resolved_ref) =
            parse_deferral_update(raw, "--resolve-deferral", "RESOLVED_REF")?;
        let finding = deferred_findings
            .iter_mut()
            .rev()
            .find(|finding| {
                finding.category == category
                    && finding.deferred_cycle == deferred_cycle
                    && is_active_deferred_finding(finding)
            })
            .ok_or_else(|| {
                format!(
                    "no active deferred finding found for category '{}' deferred in cycle {} to resolve",
                    category, deferred_cycle
                )
            })?;
        finding.resolved = true;
        finding.resolved_ref = Some(resolved_ref);
    }

    if state.get("deferred_findings").is_none() && deferred_findings == existing_snapshot {
        return Ok((None, warnings));
    }

    let value = serde_json::to_value(&deferred_findings)
        .map_err(|error| format!("failed to serialize deferred_findings patch: {}", error))?;
    Ok((
        Some(PatchUpdate {
            path: "/deferred_findings".to_string(),
            value,
        }),
        warnings,
    ))
}

/// Active deferred findings are the entries that remain unresolved.
fn is_active_deferred_finding(finding: &DeferredFinding) -> bool {
    !finding.resolved
}

fn parse_deferral_update(
    raw: &str,
    flag_name: &str,
    trailing_field_name: &str,
) -> Result<(String, u64, String), String> {
    parse_category_cycle_text_update(
        raw,
        flag_name,
        "DEFERRED_CYCLE",
        "deferred cycle",
        trailing_field_name,
    )
}

fn parse_chronic_rollback(raw: &str) -> Result<(String, u64, String), String> {
    parse_category_cycle_text_update(
        raw,
        "--rollback-chronic-category",
        "PRIOR_VC",
        "prior verification cycle",
        "RATIONALE",
    )
}

fn parse_category_cycle_text_update(
    raw: &str,
    flag_name: &str,
    cycle_component_name: &str,
    cycle_field_name: &str,
    trailing_field_name: &str,
) -> Result<(String, u64, String), String> {
    let mut parts = raw.splitn(3, ':');
    let category_raw = parts.next().unwrap_or_default();
    let cycle_raw = parts.next().unwrap_or_default();
    let trailing_value_raw = parts.next().unwrap_or_default();
    if category_raw.is_empty() || cycle_raw.is_empty() || trailing_value_raw.is_empty() {
        return Err(format!(
            "invalid {} '{}': expected CATEGORY:{}:{}",
            flag_name, raw, cycle_component_name, trailing_field_name
        ));
    }

    let category = normalize_category(category_raw).ok_or_else(|| {
        format!(
            "invalid {} category '{}': category must normalize to a non-empty slug",
            flag_name, category_raw
        )
    })?;
    let cycle = cycle_raw.parse::<u64>().map_err(|_| {
        format!(
            "invalid {} {} '{}': expected an unsigned integer",
            flag_name, cycle_field_name, cycle_raw
        )
    })?;
    let trailing_value = trailing_value_raw.trim();
    if trailing_value.is_empty() {
        return Err(format!(
            "invalid {} '{}': {} must be non-empty",
            flag_name, raw, trailing_field_name
        ));
    }

    Ok((category, cycle, trailing_value.to_string()))
}

fn insert_missing_top_level_path(state: &mut Value, path: &str, value: Value) -> bool {
    let Some(key) = path.strip_prefix('/') else {
        return false;
    };
    if key.contains('/') {
        return false;
    }

    let Some(object) = state.as_object_mut() else {
        return false;
    };
    object.insert(key.to_string(), value);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    const SAMPLE_REVIEW: &str = r#"# Cycle 162 Review

## Findings

1. **State consistency drift remains visible.**
   Category: Data Integrity
2. **Field freshness gaps continue in one path.**
   Category: Process Integrity
3. **Arithmetic checks now pass for metrics.**

## Recommendations

1. Automate more state updates.

## Complacency score

**Complacency Score: 2/5**
"#;

    const CYCLE_170_REVIEW: &str = r#"# Cycle 170 Review

## Findings

1. **`review_agent` changed this cycle, but its freshness marker still says it did not.**  
   Category: review-agent-freshness-drift  
   The worklog says cycle 170 manually corrected the review history entry after `process-review` produced bad category output (`docs/worklog/2026-03-07/045200-hundred-seventieth-orchestrator-cycle.md:23-29`), and the current history entry for cycle 169 reflects that correction (`docs/state.json:1329-1344`).

2. **The cycle fixed the rate bug in code, but only after two more rounds of manual state surgery.**  
   Category: reactive-manual-repair  
   The journal is candid that this was the third straight cycle with a manual `dispatch_to_pr_rate` repair (`docs/journal/2026-03-07.md:65-68`), and the worklog records two manual corrections in the same cycle before PR #637 merged (`docs/worklog/2026-03-07/045200-hundred-seventieth-orchestrator-cycle.md:23-29`).

3. **The publish-gate status was refreshed as current-cycle state, but its divergence evidence still points at cycle 169.**  
   Category: publish-gate-evidence-reuse  
   The worklog's current-state section says the publish gate is "FULLY CLEARED" and that there is "No source divergence" (`docs/worklog/2026-03-07/045200-hundred-seventieth-orchestrator-cycle.md:37-41`).

## Recommendations

1. Whenever `docs/state.json` is edited manually, update the matching `field_inventory` freshness entry in the same change or route the change through a tool that does it automatically.

## Complacency score

3/5 — this cycle made real improvements and the journal is genuinely self-critical rather than formulaic, but it still normalized avoidable manual state repair and let some evidence/freshness bookkeeping lag behind the actual work.
"#;

    const CYCLE_196_REVIEW: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../docs/reviews/cycle-196.md"
    ));
    const CYCLE_197_REVIEW: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../docs/reviews/cycle-197.md"
    ));
    const CYCLE_205_REVIEW: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../docs/reviews/cycle-205.md"
    ));

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--review-file"));
        assert!(help.contains("--review-issue"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--actioned"));
        assert!(help.contains("--deferred"));
        assert!(help.contains("--dispatch-created"));
        assert!(help.contains("--actioned-failed"));
        assert!(help.contains("--verified-resolved"));
        assert!(help.contains("--ignored"));
        assert!(help.contains("--disposition"));
        assert!(help.contains("--lenient"));
        assert!(help.contains("--note"));
        assert!(help.contains("--drop-deferral"));
        assert!(help.contains("--resolve-deferral"));
        assert!(help.contains("--update-chronic-category"));
        assert!(help.contains("--update-chronic-pr"));
        assert!(help.contains("--update-chronic-cycle"));
        assert!(help.contains("--update-chronic-rationale"));
        assert!(help.contains("--rollback-chronic-category"));
    }

    #[test]
    fn cli_accepts_lenient_flag() {
        assert!(Cli::try_parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--review-issue",
            "2388",
            "--lenient",
        ])
        .is_ok());
    }

    #[test]
    fn cli_parses_new_disposition_flags() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--review-issue",
            "2388",
            "--dispatch-created",
            "2",
            "--actioned-failed",
            "3",
            "--verified-resolved",
            "4",
            "--disposition",
            "data-integrity:dispatch_created",
            "--disposition",
            "process-integrity:actioned_failed",
        ]);

        assert_eq!(cli.dispatch_created, 2);
        assert_eq!(cli.actioned_failed, 3);
        assert_eq!(cli.verified_resolved, 4);
        assert_eq!(cli.review_issue, Some(2388));
        assert_eq!(
            cli.finding_dispositions,
            vec![
                "data-integrity:dispatch_created".to_string(),
                "process-integrity:actioned_failed".to_string()
            ]
        );
    }

    #[test]
    fn cli_parses_deferral_update_flags() {
        let cli = Cli::parse_from([
            "process-review",
            "--drop-deferral",
            "journal-quality:464:awaiting Eva response",
            "--resolve-deferral",
            "worklog-accuracy:468:docs/reviews/cycle-470.md",
        ]);

        assert_eq!(
            cli.drop_deferrals,
            vec!["journal-quality:464:awaiting Eva response".to_string()]
        );
        assert_eq!(
            cli.resolve_deferrals,
            vec!["worklog-accuracy:468:docs/reviews/cycle-470.md".to_string()]
        );
    }

    #[test]
    fn cli_accepts_update_chronic_without_review_file() {
        let cli = Cli::parse_from([
            "process-review",
            "--update-chronic-category",
            "worklog-accuracy",
            "--update-chronic-pr",
            "2266",
            "--update-chronic-cycle",
            "460",
        ]);

        assert_eq!(cli.review_file, None);
        assert_eq!(
            cli.update_chronic_categories,
            vec!["worklog-accuracy".to_string()]
        );
        assert_eq!(cli.update_chronic_prs, vec![2266]);
        assert_eq!(cli.update_chronic_cycle, Some(460));
    }

    #[test]
    fn cli_accepts_rollback_chronic_without_review_file() {
        let cli = Cli::parse_from([
            "process-review",
            "--rollback-chronic-category",
            "receipt-integrity:461:rollback rationale",
        ]);

        assert_eq!(cli.review_file, None);
        assert_eq!(
            cli.rollback_chronic_categories,
            vec!["receipt-integrity:461:rollback rationale".to_string()]
        );
    }

    #[test]
    fn run_rejects_review_processing_without_review_issue() {
        let repo_root = write_temp_state_repo(json!({
            "last_cycle": {"number": 500},
            "cycle_phase": {"cycle": 500},
            "review_agent": {
                "last_review_cycle": 499,
                "history": []
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 499"}
                }
            }
        }));
        init_temp_git_repo(&repo_root);
        let review_dir = repo_root.join("docs/reviews");
        fs::create_dir_all(&review_dir).expect("review directory should exist");
        fs::write(
            review_dir.join("cycle-500.md"),
            r#"# Cycle 500 Review

## Findings

1. **[review-accounting] Review accounting finding**

## Complacency score

2/5
"#,
        )
        .expect("review file should be written");

        let cli = Cli::parse_from([
            "process-review",
            "--repo-root",
            repo_root.to_str().expect("repo path should be valid UTF-8"),
            "--review-file",
            "docs/reviews/cycle-500.md",
            "--actioned",
            "1",
        ]);

        assert_eq!(
            run(cli),
            Err(format!(
                "--review-issue is required when processing review file {}",
                review_dir.join("cycle-500.md").display()
            ))
        );
    }

    #[test]
    fn score_extraction_reads_complacency_section() {
        let markdown = r#"## Recommendations

1. Keep the previous 5/5 note out of the score parser.

## Complacency

2/5 — actual score
"#;
        assert_eq!(extract_score(markdown), Some(2));
    }

    #[test]
    fn score_extraction_ignores_recommendation_scores_before_complacency_section() {
        let markdown = r#"## Recommendations

3. Stop recording pipeline status as a bare "5/5" unless the evidence is current.

## Complacency score

4/5 — cycle 183 did real work and still left follow-up behind.
"#;
        assert_eq!(extract_score(markdown), Some(4));
    }

    #[test]
    fn finding_count_counts_only_numbered_bold_findings() {
        assert_eq!(extract_finding_count(SAMPLE_REVIEW), Some(3));
    }

    #[test]
    fn finding_count_uses_cycle_170_review_format() {
        assert_eq!(extract_finding_count(CYCLE_170_REVIEW), Some(3));
        assert_eq!(extract_score(CYCLE_170_REVIEW), Some(3));
    }

    #[test]
    fn finding_count_requires_bold_numbered_findings() {
        let markdown = r#"## Findings
1. **One**
2. **Two**
## Recommendations
1. R
"#;
        assert_eq!(extract_finding_count(markdown), Some(2));
    }

    #[test]
    fn numbered_finding_number_supports_hash_prefixed_findings() {
        assert_eq!(
            numbered_finding_number("## 1. [worklog-accuracy] Finding title"),
            Some(1)
        );
    }

    #[test]
    fn category_extraction_normalizes_values() {
        let categories = extract_categories(CYCLE_170_REVIEW);
        assert_eq!(
            categories,
            vec![
                "publish-gate-evidence-reuse".to_string(),
                "reactive-manual-repair".to_string(),
                "review-agent-freshness-drift".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_ignores_titles_without_category_lines() {
        let markdown = r#"## Findings

1. **Cycle 123 closes with a false all-green narrative: the repository currently fails 2 of the 9 state invariants.**

## Complacency score

**2/5**
"#;

        assert!(extract_categories(markdown).is_empty());
    }

    #[test]
    fn category_extraction_reads_inline_heading_category() {
        let markdown = r#"## Findings

1. **[metrics-ownership] Finding title**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec!["metrics-ownership".to_string()]
        );
    }

    #[test]
    fn category_extraction_reads_hash_prefixed_heading_category() {
        let markdown = r#"## Findings

## 1. [worklog-accuracy] First finding

## 2. [tooling-contract] Second finding
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "tooling-contract".to_string(),
                "worklog-accuracy".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_supports_mixed_inline_and_category_line_formats() {
        let markdown = r#"## Findings

1. **[metrics-ownership] Inline finding**

2. **Separate category finding**
   Category: Review Accounting
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "metrics-ownership".to_string(),
                "review-accounting".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_collects_multiple_inline_categories() {
        let markdown = r#"## Findings

1. **[metrics-ownership] First finding**

2. **[tooling-contract] Second finding**

3. **[review-accounting] Third finding**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "metrics-ownership".to_string(),
                "review-accounting".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_uses_first_inline_bracket_pair_only() {
        let markdown = r#"## Findings

1. **[category-name] Title with [other] brackets**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec!["category-name".to_string()]
        );
    }

    #[test]
    fn category_extraction_parses_cycle_196_artifact_with_inline_categories() {
        assert_eq!(
            extract_categories(CYCLE_196_REVIEW),
            vec![
                "metrics-ownership".to_string(),
                "process-adherence".to_string(),
                "review-accounting".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_deduplicates_short_categories() {
        let markdown = r#"## Findings

1. **State consistency remains visible across multiple validation surfaces.**
   Category: Data Integrity
2. **Data integrity remains the priority across all repository state accounting checks.**
   Category: Data Integrity
3. **Process integrity needs follow-up across several long-running workflow maintenance paths.**
   Category: Process Integrity
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ]
        );
    }

    #[test]
    fn parsing_ignores_code_blocks_and_paths() {
        let markdown = r#"## Findings

1. **Real finding**
   Category: First Category
   Description with a real path: `docs/reviews/cycle-170.md`

```text
1. **Not a real finding**
   Category: bogus-category
/home/runner/work/schema-org-json-ld/schema-org-json-ld/tools/rust/crates/process-review/src/main.rs
```

2. **Second real finding**
   Category: Second Category

## Recommendations

1. Leave the code block alone.
"#;

        assert_eq!(extract_finding_count(markdown), Some(2));
        assert_eq!(
            extract_categories(markdown),
            vec!["first-category".to_string(), "second-category".to_string()]
        );
    }

    #[test]
    fn finding_count_supports_mixed_hash_prefixed_findings() {
        let markdown = r#"## Findings

1. **[metrics-ownership] Legacy finding**

## 2. [worklog-accuracy] Hash-prefixed finding

## 3. [tooling-contract] Another hash-prefixed finding

## Recommendations

1. Leave the parser alone.
"#;

        assert_eq!(count_numbered_findings(markdown), 3);
    }

    #[test]
    fn finding_count_falls_back_without_findings_header() {
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] First finding

## 2. [tooling-contract] Second finding

## Complacency score

**4/5**
"#;

        assert_eq!(extract_finding_count(markdown), Some(2));
        assert_eq!(extract_score(markdown), Some(4));
    }

    #[test]
    fn category_extraction_falls_back_without_findings_header() {
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] First finding

## 2. [tooling-contract] Second finding

## Complacency score

**4/5**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "review-evidence".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn history_entry_construction_uses_cli_overrides() {
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 8,
            complacency_score: 2,
            categories: vec!["data-integrity".to_string()],
        };
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--dispatch-created",
            "2",
            "--actioned-failed",
            "1",
            "--verified-resolved",
            "2",
            "--ignored",
            "1",
            "--note",
            "triaged",
            "--disposition",
            "data-integrity:actioned",
            "--disposition",
            "data-integrity:verified_resolved",
            "--disposition",
            "data-integrity:ignored",
            "--disposition",
            "data-integrity:dispatch_created",
            "--disposition",
            "data-integrity:dispatch_created",
            "--disposition",
            "data-integrity:actioned_failed",
            "--disposition",
            "data-integrity:deferred",
            "--disposition",
            "data-integrity:verified_resolved",
        ]);

        let entry = build_history_entry(
            &parsed,
            &cli,
            2388,
            validate_dispositions(&cli, &parsed).expect("dispositions should validate"),
        );
        assert_eq!(entry.cycle, 162);
        assert_eq!(entry.finding_count, 8);
        assert_eq!(entry.complacency_score, 2);
        assert_eq!(entry.review_issue, Some(2388));
        assert_eq!(entry.actioned, 1);
        assert_eq!(entry.deferred, 1);
        assert_eq!(entry.dispatch_created, 2);
        assert_eq!(entry.actioned_failed, 1);
        assert_eq!(entry.verified_resolved, 2);
        assert_eq!(entry.ignored, 1);
        assert_eq!(entry.note.as_deref(), Some("triaged"));
        assert_eq!(
            entry.finding_dispositions,
            vec![
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "actioned".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "verified_resolved".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "ignored".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "dispatch_created".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "dispatch_created".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "actioned_failed".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "deferred".to_string(),
                },
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "verified_resolved".to_string(),
                },
            ]
        );
    }

    #[test]
    fn history_entry_serialization_omits_zero_valued_new_fields() {
        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 3,
            complacency_score: 1,
            review_issue: None,
            categories: vec!["state-consistency".to_string()],
            actioned: 1,
            deferred: 1,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 1,
            note: None,
            finding_dispositions: Vec::new(),
        };

        let value = serde_json::to_value(&entry).expect("history entry should serialize");
        let object = value
            .as_object()
            .expect("history entry should be an object");
        assert!(!object.contains_key("dispatch_created"));
        assert!(!object.contains_key("actioned_failed"));
        assert!(!object.contains_key("verified_resolved"));
        assert!(!object.contains_key("finding_dispositions"));
    }

    #[test]
    fn history_entry_serialization_includes_non_zero_new_fields() {
        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 3,
            complacency_score: 1,
            review_issue: Some(2393),
            categories: vec!["state-consistency".to_string()],
            actioned: 1,
            deferred: 0,
            dispatch_created: 1,
            actioned_failed: 1,
            verified_resolved: 1,
            ignored: 0,
            note: None,
            finding_dispositions: vec![FindingDisposition {
                category: "state-consistency".to_string(),
                disposition: "dispatch_created".to_string(),
            }],
        };

        let value = serde_json::to_value(&entry).expect("history entry should serialize");
        let object = value
            .as_object()
            .expect("history entry should be an object");
        assert_eq!(object.get("dispatch_created"), Some(&json!(1)));
        assert_eq!(object.get("actioned_failed"), Some(&json!(1)));
        assert_eq!(object.get("verified_resolved"), Some(&json!(1)));
        assert_eq!(object.get("review_issue"), Some(&json!(2393)));
        assert_eq!(
            object.get("finding_dispositions"),
            Some(&json!([{
                "category": "state-consistency",
                "disposition": "dispatch_created"
            }]))
        );
    }

    #[test]
    fn disposition_validation_rejects_default_counts_when_findings_exist() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
        ]);

        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        let error = validate_dispositions(&cli, &parsed).expect_err("validation should fail");
        assert!(error.contains("expected 3"));
        assert!(error.contains("got 0"));
    }

    #[test]
    fn disposition_validation_passes_when_counts_sum_correctly() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--dispatch-created",
            "1",
            "--actioned-failed",
            "1",
            "--verified-resolved",
            "1",
        ]);

        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 5,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        assert_eq!(validate_dispositions(&cli, &parsed), Ok(Vec::new()));
    }

    #[test]
    fn disposition_validation_accepts_all_dispatch_created_findings() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--dispatch-created",
            "3",
        ]);

        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec!["data-integrity".to_string()],
        };

        assert_eq!(validate_dispositions(&cli, &parsed), Ok(Vec::new()));
    }

    #[test]
    fn cli_rejects_removed_skip_disposition_check_flag() {
        let removed_flag = format!("--{}-{}-{}", "skip", "disposition", "check");
        let error = Cli::try_parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            removed_flag.as_str(),
        ])
        .expect_err("removed flag should be rejected");

        let rendered = error.to_string();
        assert!(rendered.contains("unexpected argument"));
        assert!(rendered.contains(&removed_flag));
    }

    #[test]
    fn disposition_validation_allows_zero_findings_with_zero_counts() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
        ]);

        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 0,
            complacency_score: 2,
            categories: Vec::new(),
        };

        assert_eq!(validate_dispositions(&cli, &parsed), Ok(Vec::new()));
    }

    #[test]
    fn disposition_validation_rejects_non_default_mismatch() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
        ]);

        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec!["data-integrity".to_string()],
        };

        let error = validate_dispositions(&cli, &parsed).expect_err("validation should fail");
        assert!(error.contains("expected 3"));
        assert!(error.contains("got 1"));
    }

    #[test]
    fn disposition_validation_accepts_matching_per_finding_dispositions() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--ignored",
            "1",
            "--disposition",
            "data-integrity:actioned",
            "--disposition",
            "process-integrity:deferred",
            "--disposition",
            "process-integrity:ignored",
        ]);
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        let dispositions =
            validate_dispositions(&cli, &parsed).expect("dispositions should validate");
        assert_eq!(
            dispositions,
            vec![
                FindingDisposition {
                    category: "data-integrity".to_string(),
                    disposition: "actioned".to_string(),
                },
                FindingDisposition {
                    category: "process-integrity".to_string(),
                    disposition: "deferred".to_string(),
                },
                FindingDisposition {
                    category: "process-integrity".to_string(),
                    disposition: "ignored".to_string(),
                },
            ]
        );
    }

    #[test]
    fn disposition_validation_rejects_per_finding_count_mismatch() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "2",
            "--deferred",
            "1",
            "--disposition",
            "data-integrity:actioned",
            "--disposition",
            "process-integrity:deferred",
        ]);
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        let error = validate_dispositions(&cli, &parsed).expect_err("validation should fail");
        assert!(error.contains("expected 3 entries"));
        assert!(error.contains("got 2"));
    }

    #[test]
    fn disposition_validation_rejects_category_not_in_review() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--ignored",
            "1",
            "--disposition",
            "data-integrity:actioned",
            "--disposition",
            "unknown-category:deferred",
            "--disposition",
            "process-integrity:ignored",
        ]);
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        let error = validate_dispositions(&cli, &parsed).expect_err("validation should fail");
        assert!(error.contains("unknown-category"));
        assert!(error.contains("does not appear in the review file categories"));
    }

    #[test]
    fn disposition_validation_rejects_aggregate_mismatch_from_per_finding_values() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "2",
            "--deferred",
            "1",
            "--disposition",
            "data-integrity:actioned",
            "--disposition",
            "process-integrity:deferred",
            "--disposition",
            "process-integrity:ignored",
        ]);
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        let error = validate_dispositions(&cli, &parsed).expect_err("validation should fail");
        assert!(error.contains("aggregate counts do not match"));
        assert!(error.contains("ignored=1"));
    }

    #[test]
    fn disposition_validation_is_backward_compatible_without_per_finding_flags() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--ignored",
            "1",
        ]);
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 3,
            complacency_score: 2,
            categories: vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ],
        };

        let dispositions =
            validate_dispositions(&cli, &parsed).expect("backward-compatible mode should pass");
        assert!(dispositions.is_empty());
    }

    #[test]
    fn recognized_categories_pass_validation_silently() {
        let repo_root = write_temp_state_repo(json!({
            "review_agent": {
                "history": [],
                "chronic_category_responses": {"entries": []}
            }
        }));

        let warnings = validate_categories(&repo_root, &["review-accounting".to_string()])
            .expect("validation should succeed");
        assert!(warnings.is_empty());
    }

    #[test]
    fn unrecognized_categories_emit_warning_without_failing() {
        let repo_root = write_temp_state_repo(json!({
            "review_agent": {
                "history": [],
                "chronic_category_responses": {"entries": []}
            }
        }));

        let warnings = validate_categories(&repo_root, &["brand-new-category".to_string()])
            .expect("validation should succeed");
        assert_eq!(
            warnings,
            vec!["WARNING: unrecognized review category: brand-new-category".to_string()]
        );
    }

    #[test]
    fn chronic_category_responses_categories_are_treated_as_known() {
        let repo_root = write_temp_state_repo(json!({
            "review_agent": {
                "history": [],
                "chronic_category_responses": {
                    "entries": [
                        {"category": "Custom Chronic Category"}
                    ]
                }
            }
        }));

        let warnings = validate_categories(&repo_root, &["custom-chronic-category".to_string()])
            .expect("validation should succeed");
        assert!(warnings.is_empty());
    }

    #[test]
    fn known_review_categories_from_state_combines_builtin_history_and_chronic_categories() {
        let state: StateJson = serde_json::from_value(json!({
            "review_agent": {
                "history": [
                    {"categories": ["History Category"], "actioned": 0, "deferred": 0, "ignored": 0, "finding_count": 1, "complacency_score": 1, "cycle": 1}
                ],
                "chronic_category_responses": {
                    "entries": [
                        {"category": "Chronic Category"}
                    ]
                }
            }
        }))
        .expect("state should parse");

        let categories =
            known_review_categories_from_state(&state).expect("category extraction should work");

        assert!(categories.contains("review-accounting"));
        assert!(categories.contains("history-category"));
        assert!(categories.contains("chronic-category"));
    }

    #[test]
    fn known_review_categories_from_state_requires_review_agent() {
        let state = StateJson::default();

        let error = known_review_categories_from_state(&state)
            .expect_err("missing review_agent should fail");
        assert!(error.contains("missing field: review_agent"));
    }

    #[test]
    fn chronic_response_categories_ignore_missing_entries_and_categories() {
        assert!(chronic_response_categories(&json!({})).is_empty());
        assert!(chronic_response_categories(&json!({"entries": [{}]})).is_empty());
        assert_eq!(
            chronic_response_categories(&json!({
                "entries": [
                    {"category": "Valid Category"},
                    {"category": 3},
                    {"other": "ignored"}
                ]
            })),
            BTreeSet::from(["valid-category".to_string()])
        );
    }

    #[test]
    fn state_patch_generation_updates_expected_paths() {
        let state = json!({
            "last_cycle": {"number": 163},
            "review_agent": {
                "last_review_cycle": 162,
                "history": [
                    {"cycle": 162, "finding_count": 7, "complacency_score": 2, "categories": ["a"], "actioned": 1, "deferred": 1, "ignored": 5}
                ]
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 162"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 3,
            complacency_score: 1,
            review_issue: None,
            categories: vec!["state-consistency".to_string()],
            actioned: 1,
            deferred: 1,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 1,
            note: None,
            finding_dispositions: Vec::new(),
        };

        let (patch, warnings) =
            build_state_patch(&state, 163, 163, &entry, &[], &[]).expect("patch should build");
        assert!(warnings.is_empty());
        assert_eq!(patch.len(), 3);
        assert_eq!(patch[0].path, "/review_agent/last_review_cycle");
        assert_eq!(patch[1].path, "/review_agent/history");
        assert_eq!(
            patch[2].path,
            "/field_inventory/fields/review_agent/last_refreshed"
        );

        let history = patch[1]
            .value
            .as_array()
            .expect("history value should be array");
        assert_eq!(history.len(), 2);
        assert_eq!(
            history.last().and_then(|value| value.get("cycle")),
            Some(&json!(163))
        );
    }

    #[test]
    fn state_patch_updates_in_place_when_cycle_already_exists() {
        let state = json!({
            "last_cycle": {"number": 163},
            "review_agent": {
                "last_review_cycle": 163,
                "history": [
                    {"cycle": 162, "finding_count": 5, "complacency_score": 2, "categories": ["a"], "actioned": 2, "deferred": 2, "ignored": 1},
                    {"cycle": 163, "finding_count": 3, "complacency_score": 2, "categories": ["state-consistency"], "actioned": 0, "deferred": 3, "ignored": 0}
                ]
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 162"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 3,
            complacency_score: 2,
            review_issue: None,
            categories: vec!["state-consistency".to_string()],
            actioned: 0,
            deferred: 1,
            dispatch_created: 2,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 0,
            note: Some("Updated dispositions after dispatches created".to_string()),
            finding_dispositions: Vec::new(),
        };

        let (patch, warnings) =
            build_state_patch(&state, 163, 163, &entry, &[], &[]).expect("patch should build");

        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("cycle 163 history entry already existed"),
            "expected duplicate warning, got: {:?}",
            warnings[0]
        );

        let history = patch[1]
            .value
            .as_array()
            .expect("history value should be array");

        // Should still be 2 entries (in-place replacement, not appended)
        assert_eq!(
            history.len(),
            2,
            "history length should not grow on duplicate"
        );

        let updated = history
            .iter()
            .find(|e| e.get("cycle").and_then(Value::as_u64) == Some(163))
            .expect("cycle 163 entry must be present");
        assert_eq!(updated.get("deferred").and_then(Value::as_u64), Some(1));
        assert_eq!(
            updated.get("dispatch_created").and_then(Value::as_u64),
            Some(2)
        );
    }

    #[test]
    fn state_patch_emits_warning_message_on_duplicate_cycle() {
        let state = json!({
            "last_cycle": {"number": 200},
            "review_agent": {
                "last_review_cycle": 200,
                "history": [
                    {"cycle": 200, "finding_count": 1, "complacency_score": 1, "categories": ["x"], "actioned": 1, "deferred": 0, "ignored": 0}
                ]
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 199"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 200,
            finding_count: 1,
            complacency_score: 1,
            review_issue: None,
            categories: vec!["x".to_string()],
            actioned: 0,
            deferred: 1,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 0,
            note: None,
            finding_dispositions: Vec::new(),
        };

        let (_patch, warnings) =
            build_state_patch(&state, 200, 200, &entry, &[], &[]).expect("patch should build");

        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("200"),
            "warning must reference the cycle number"
        );
        assert!(
            warnings[0].contains("replaced"),
            "warning must say entry was replaced"
        );
    }

    #[test]
    fn state_patch_generation_anchors_deferred_findings_to_review_cycle() {
        let state = json!({
            "last_cycle": {"number": 163},
            "review_agent": {
                "last_review_cycle": 162,
                "history": []
            },
            "deferred_findings": [{
                "category": "review-accounting",
                "deferred_cycle": 158,
                "deadline_cycle": 163,
                "resolved": false
            }],
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 162"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 2,
            complacency_score: 2,
            review_issue: None,
            categories: vec![
                "review-accounting".to_string(),
                "tooling-contract".to_string(),
            ],
            actioned: 1,
            deferred: 1,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 0,
            note: None,
            finding_dispositions: vec![
                FindingDisposition {
                    category: "review-accounting".to_string(),
                    disposition: "actioned".to_string(),
                },
                FindingDisposition {
                    category: "tooling-contract".to_string(),
                    disposition: "deferred".to_string(),
                },
            ],
        };

        let (patch, warnings) =
            build_state_patch(&state, 163, 164, &entry, &[], &[]).expect("patch should build");

        assert!(warnings.is_empty());
        assert_eq!(patch.len(), 4);
        assert_eq!(patch[2].path, "/deferred_findings");

        assert_eq!(
            patch[2].value,
            json!([
                {
                    "category": "review-accounting",
                    "deferred_cycle": 158,
                    "deadline_cycle": 163,
                    "resolved": true,
                    "resolved_ref": "docs/reviews/cycle-163.md"
                },
                {
                    "category": "tooling-contract",
                    "deferred_cycle": 163,
                    "deadline_cycle": 168,
                    "resolved": false
                }
            ])
        );
    }

    #[test]
    fn state_patch_generation_marks_verified_resolved_findings_as_resolved() {
        let state = json!({
            "last_cycle": {"number": 164},
            "review_agent": {
                "last_review_cycle": 163,
                "history": []
            },
            "deferred_findings": [{
                "category": "tooling-contract",
                "deferred_cycle": 163,
                "deadline_cycle": 168,
                "resolved": false
            }],
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 163"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 164,
            finding_count: 1,
            complacency_score: 2,
            review_issue: None,
            categories: vec!["tooling-contract".to_string()],
            actioned: 0,
            deferred: 0,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 1,
            ignored: 0,
            note: None,
            finding_dispositions: vec![FindingDisposition {
                category: "tooling-contract".to_string(),
                disposition: "verified_resolved".to_string(),
            }],
        };

        let (patch, warnings) =
            build_state_patch(&state, 164, 165, &entry, &[], &[]).expect("patch should build");

        assert!(warnings.is_empty());
        assert_eq!(patch[2].path, "/deferred_findings");
        assert_eq!(
            patch[2].value,
            json!([{
                "category": "tooling-contract",
                "deferred_cycle": 163,
                "deadline_cycle": 168,
                "resolved": true,
                "resolved_ref": "docs/reviews/cycle-164.md"
            }])
        );
    }

    #[test]
    fn state_patch_generation_warns_instead_of_creating_duplicate_unresolved_deferred_entry() {
        let state = json!({
            "last_cycle": {"number": 163},
            "review_agent": {
                "last_review_cycle": 162,
                "history": []
            },
            "deferred_findings": [{
                "category": "review-accounting",
                "deferred_cycle": 160,
                "deadline_cycle": 165,
                "resolved": false
            }],
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 162"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 1,
            complacency_score: 2,
            review_issue: None,
            categories: vec!["review-accounting".to_string()],
            actioned: 0,
            deferred: 1,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 0,
            note: None,
            finding_dispositions: vec![FindingDisposition {
                category: "review-accounting".to_string(),
                disposition: "deferred".to_string(),
            }],
        };

        let (patch, warnings) =
            build_state_patch(&state, 163, 164, &entry, &[], &[]).expect("patch should build");

        assert_eq!(patch[2].path, "/deferred_findings");
        assert_eq!(
            patch[2].value,
            json!([{
                "category": "review-accounting",
                "deferred_cycle": 160,
                "deadline_cycle": 165,
                "resolved": false
            }])
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("review-accounting"));
        assert!(warnings[0].contains("already has an unresolved deferred finding"));
    }

    #[test]
    fn drop_deferral_sets_dropped_rationale_on_matching_entry() {
        let state = json!({
            "deferred_findings": [
                {
                    "category": "journal-quality",
                    "deferred_cycle": 464,
                    "deadline_cycle": 469,
                    "resolved": false
                },
                {
                    "category": "journal-quality",
                    "deferred_cycle": 465,
                    "deadline_cycle": 470,
                    "resolved": false
                }
            ]
        });

        let (patch, warnings) = deferred_findings_patch(
            &state,
            None,
            None,
            &["journal-quality:464:awaiting Eva response".to_string()],
            &[],
        )
        .expect("drop update should succeed");

        assert!(warnings.is_empty());
        assert_eq!(
            patch.expect("patch should be generated").value,
            json!([
                {
                    "category": "journal-quality",
                    "deferred_cycle": 464,
                    "deadline_cycle": 469,
                    "resolved": true,
                    "resolved_ref": "dropped: awaiting Eva response",
                    "dropped_rationale": "awaiting Eva response"
                },
                {
                    "category": "journal-quality",
                    "deferred_cycle": 465,
                    "deadline_cycle": 470,
                    "resolved": false
                }
            ])
        );
    }

    #[test]
    fn dropped_deferral_is_not_an_active_deferred_finding() {
        let finding = DeferredFinding {
            category: "journal-quality".to_string(),
            deferred_cycle: 464,
            deadline_cycle: 469,
            resolved: true,
            resolved_ref: Some("dropped: awaiting Eva response".to_string()),
            dropped_rationale: Some("awaiting Eva response".to_string()),
        };

        assert!(!is_active_deferred_finding(&finding));
    }

    #[test]
    fn drop_deferral_errors_when_no_matching_entry_exists() {
        let state = json!({
            "deferred_findings": [{
                "category": "journal-quality",
                "deferred_cycle": 464,
                "deadline_cycle": 469,
                "resolved": true,
                "resolved_ref": "docs/reviews/cycle-470.md"
            }]
        });

        let error = deferred_findings_patch(
            &state,
            None,
            None,
            &["journal-quality:464:awaiting Eva response".to_string()],
            &[],
        )
        .expect_err("drop update should fail");

        assert!(error.contains("journal-quality"));
        assert!(error.contains("464"));
        assert!(error.contains("drop"));
    }

    #[test]
    fn resolve_deferral_sets_resolved_fields_on_matching_entry() {
        let state = json!({
            "deferred_findings": [
                {
                    "category": "worklog-accuracy",
                    "deferred_cycle": 468,
                    "deadline_cycle": 473,
                    "resolved": false
                },
                {
                    "category": "worklog-accuracy",
                    "deferred_cycle": 467,
                    "deadline_cycle": 472,
                    "resolved": false
                }
            ]
        });

        let (patch, warnings) = deferred_findings_patch(
            &state,
            None,
            None,
            &[],
            &["worklog-accuracy:468:docs/reviews/cycle-470.md".to_string()],
        )
        .expect("resolve update should succeed");

        assert!(warnings.is_empty());
        assert_eq!(
            patch.expect("patch should be generated").value,
            json!([
                {
                    "category": "worklog-accuracy",
                    "deferred_cycle": 468,
                    "deadline_cycle": 473,
                    "resolved": true,
                    "resolved_ref": "docs/reviews/cycle-470.md"
                },
                {
                    "category": "worklog-accuracy",
                    "deferred_cycle": 467,
                    "deadline_cycle": 472,
                    "resolved": false
                }
            ])
        );
    }

    #[test]
    fn resolve_deferral_errors_when_no_matching_entry_exists() {
        let state = json!({
            "deferred_findings": [{
                "category": "worklog-accuracy",
                "deferred_cycle": 468,
                "deadline_cycle": 473,
                "resolved": true,
                "resolved_ref": "dropped: superseded",
                "dropped_rationale": "superseded"
            }]
        });

        let error = deferred_findings_patch(
            &state,
            None,
            None,
            &[],
            &["worklog-accuracy:468:docs/reviews/cycle-470.md".to_string()],
        )
        .expect_err("resolve update should fail");

        assert!(error.contains("worklog-accuracy"));
        assert!(error.contains("468"));
        assert!(error.contains("resolve"));
    }

    #[test]
    fn update_chronic_category_bumps_verification_cycle() {
        let mut state = chronic_state_fixture(json!([{
            "category": "foo",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 448,
            "verification_cycle": 448
        }]));

        let summary =
            update_chronic_category_responses(&mut state, &["foo".to_string()], &[], 500, None)
                .expect("update should succeed");

        assert_eq!(summary.updated_entries, 1);
        let entry = &state["review_agent"]["chronic_category_responses"]["entries"][0];
        assert_eq!(entry["updated_cycle"], json!(500));
        assert_eq!(entry["verification_cycle"], json!(500));
    }

    #[test]
    fn update_chronic_category_handles_multiple_entries_per_category() {
        let mut state = chronic_state_fixture(json!([
            {
                "category": "worklog-accuracy",
                "chosen_path": "structural-fix",
                "rationale": "existing structural",
                "updated_cycle": 448,
                "verification_cycle": 448
            },
            {
                "category": "worklog-accuracy",
                "chosen_path": "behavioral-fix",
                "rationale": "existing behavioral",
                "updated_cycle": 448,
                "verification_cycle": 448
            }
        ]));

        let summary = update_chronic_category_responses(
            &mut state,
            &["worklog-accuracy".to_string()],
            &[],
            500,
            None,
        )
        .expect("update should succeed");

        assert_eq!(summary.updated_entries, 2);
        let entries = state["review_agent"]["chronic_category_responses"]["entries"]
            .as_array()
            .expect("entries should be an array");
        assert!(entries
            .iter()
            .all(|entry| entry["updated_cycle"] == json!(500)));
        assert!(entries
            .iter()
            .all(|entry| entry["verification_cycle"] == json!(500)));
    }

    #[test]
    fn update_chronic_category_appends_pr_to_rationale() {
        let mut state = chronic_state_fixture(json!([{
            "category": "foo",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 448,
            "verification_cycle": 448
        }]));

        update_chronic_category_responses(
            &mut state,
            &["foo".to_string()],
            &[1234],
            500,
            Some("text"),
        )
        .expect("update should succeed");

        assert_eq!(
            state["review_agent"]["chronic_category_responses"]["entries"][0]["rationale"],
            json!("existing | Cycle 500: refreshed via PR(s) [#1234] — text")
        );
    }

    #[test]
    fn update_chronic_category_is_idempotent() {
        let mut state = chronic_state_fixture(json!([{
            "category": "foo",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 448,
            "verification_cycle": 448
        }]));

        update_chronic_category_responses(
            &mut state,
            &["foo".to_string()],
            &[1234],
            500,
            Some("text"),
        )
        .expect("first update should succeed");
        let once = state.clone();

        update_chronic_category_responses(
            &mut state,
            &["foo".to_string()],
            &[1234],
            500,
            Some("text"),
        )
        .expect("second update should succeed");

        assert_eq!(state, once);
    }

    #[test]
    fn update_chronic_category_bumps_field_inventory_marker() {
        let mut state = chronic_state_fixture(json!([{
            "category": "foo",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 448,
            "verification_cycle": 448
        }]));

        update_chronic_category_responses(&mut state, &["foo".to_string()], &[], 500, None)
            .expect("update should succeed");

        assert_eq!(
            state["field_inventory"]["fields"]["review_agent.chronic_category_responses"]
                ["last_refreshed"],
            json!("cycle 500")
        );
    }

    #[test]
    fn rollback_chronic_category_restores_verification_cycle() {
        let mut state = chronic_state_fixture(json!([{
            "category": "receipt-integrity",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 474,
            "verification_cycle": 474
        }]));

        let summary = rollback_chronic_category_responses(
            &mut state,
            &["receipt-integrity:461:rollback rationale".to_string()],
            500,
        )
        .expect("rollback should succeed");

        assert_eq!(summary.updated_entries, 1);
        let entry = &state["review_agent"]["chronic_category_responses"]["entries"][0];
        assert_eq!(entry["updated_cycle"], json!(500));
        assert_eq!(entry["verification_cycle"], json!(461));
    }

    #[test]
    fn rollback_chronic_category_appends_rationale_with_prior_refresh_cycle() {
        let mut state = chronic_state_fixture(json!([{
            "category": "receipt-integrity",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 474,
            "verification_cycle": 474
        }]));

        rollback_chronic_category_responses(
            &mut state,
            &["receipt-integrity:461:rollback rationale".to_string()],
            500,
        )
        .expect("rollback should succeed");

        assert_eq!(
            state["review_agent"]["chronic_category_responses"]["entries"][0]["rationale"],
            json!("existing | Cycle 500: rollback of cycle 474 refresh — rollback rationale")
        );
    }

    #[test]
    fn rollback_chronic_category_errors_when_category_missing() {
        let mut state = chronic_state_fixture(json!([{
            "category": "receipt-integrity",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 474,
            "verification_cycle": 474
        }]));

        let error = rollback_chronic_category_responses(
            &mut state,
            &["process-adherence:462:rollback rationale".to_string()],
            500,
        )
        .expect_err("rollback should fail");

        assert!(error.contains("no chronic_category_responses entries found"));
        assert!(error.contains("process-adherence"));
    }

    #[test]
    fn rollback_chronic_category_rejects_non_regressive_prior_cycle() {
        let mut state = chronic_state_fixture(json!([{
            "category": "receipt-integrity",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 474,
            "verification_cycle": 474
        }]));

        let error = rollback_chronic_category_responses(
            &mut state,
            &["receipt-integrity:474:rollback rationale".to_string()],
            500,
        )
        .expect_err("rollback should fail");

        assert!(error.contains("prior verification cycle 474"));
        assert!(error.contains("current verification_cycle 474"));
    }

    #[test]
    fn rollback_chronic_category_bumps_field_inventory_marker() {
        let mut state = chronic_state_fixture(json!([{
            "category": "receipt-integrity",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 474,
            "verification_cycle": 474
        }]));

        rollback_chronic_category_responses(
            &mut state,
            &["receipt-integrity:461:rollback rationale".to_string()],
            500,
        )
        .expect("rollback should succeed");

        assert_eq!(
            state["field_inventory"]["fields"]["review_agent.chronic_category_responses"]
                ["last_refreshed"],
            json!("cycle 500")
        );
    }

    #[test]
    fn update_chronic_category_does_nothing_when_no_flags() {
        let chronic_entries = json!([{
            "category": "worklog-accuracy",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 448,
            "verification_cycle": 448
        }]);
        let state = json!({
            "last_cycle": {"number": 500},
            "cycle_phase": {"cycle": 500},
            "review_agent": {
                "last_review_cycle": 499,
                "history": [],
                "chronic_category_responses": {
                    "entries": chronic_entries
                }
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 499"},
                    "review_agent.chronic_category_responses": {"last_refreshed": "cycle 448"}
                }
            }
        });
        let repo_root = write_temp_state_repo(state.clone());
        init_temp_git_repo(&repo_root);
        let review_dir = repo_root.join("docs/reviews");
        fs::create_dir_all(&review_dir).expect("review directory should exist");
        fs::write(
            review_dir.join("cycle-500.md"),
            r#"# Cycle 500 Review

## Findings

1. **[review-accounting] Review accounting finding**

## Complacency score

2/5
"#,
        )
        .expect("review file should be written");

        let cli = Cli::parse_from([
            "process-review",
            "--repo-root",
            repo_root.to_str().expect("repo path should be valid UTF-8"),
            "--review-file",
            "docs/reviews/cycle-500.md",
            "--review-issue",
            "900",
            "--actioned",
            "1",
        ]);

        run(cli).expect("review-only run should succeed");

        let updated_state = read_state_value(&repo_root).expect("state should be readable");
        assert_eq!(
            updated_state.pointer("/review_agent/chronic_category_responses"),
            state.pointer("/review_agent/chronic_category_responses")
        );
        assert_eq!(
            updated_state
                .pointer("/field_inventory/fields/review_agent.chronic_category_responses"),
            state.pointer("/field_inventory/fields/review_agent.chronic_category_responses")
        );
    }

    #[test]
    fn parse_review_extracts_cycle_from_filename() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let parsed = parse_review(path, SAMPLE_REVIEW, true).expect("parse should succeed");
        assert_eq!(parsed.cycle, 162);
        assert_eq!(parsed.complacency_score, 2);
        assert_eq!(parsed.finding_count, 3);
    }

    #[test]
    fn run_accepts_drop_deferral_without_review_file() {
        let repo_root = write_temp_state_repo(json!({
            "last_cycle": {"number": 500},
            "cycle_phase": {"cycle": 500},
            "review_agent": {
                "last_review_cycle": 499,
                "history": []
            },
            "deferred_findings": [{
                "category": "journal-quality",
                "deferred_cycle": 464,
                "deadline_cycle": 469,
                "resolved": false
            }],
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 499"}
                }
            }
        }));
        init_temp_git_repo(&repo_root);

        let cli = Cli::parse_from([
            "process-review",
            "--repo-root",
            repo_root.to_str().expect("repo path should be valid UTF-8"),
            "--drop-deferral",
            "journal-quality:464:awaiting Eva response",
        ]);

        run(cli).expect("deferral-only run should succeed");

        let updated_state = read_state_value(&repo_root).expect("state should be readable");
        assert_eq!(
            updated_state.pointer("/deferred_findings/0/dropped_rationale"),
            Some(&json!("awaiting Eva response"))
        );
        assert_eq!(
            updated_state.pointer("/deferred_findings/0/resolved"),
            Some(&json!(true))
        );
        assert_eq!(
            updated_state.pointer("/deferred_findings/0/resolved_ref"),
            Some(&json!("dropped: awaiting Eva response"))
        );
        assert_eq!(
            updated_state.pointer("/review_agent/history"),
            Some(&json!([]))
        );
    }

    #[test]
    fn run_accepts_rollback_chronic_without_review_file() {
        let repo_root = write_temp_state_repo(chronic_state_fixture(json!([{
            "category": "receipt-integrity",
            "chosen_path": "structural-fix",
            "rationale": "existing",
            "updated_cycle": 474,
            "verification_cycle": 474
        }])));
        init_temp_git_repo(&repo_root);

        let cli = Cli::parse_from([
            "process-review",
            "--repo-root",
            repo_root.to_str().expect("repo path should be valid UTF-8"),
            "--rollback-chronic-category",
            "receipt-integrity:461:rollback rationale",
        ]);

        run(cli).expect("rollback-only run should succeed");

        let updated_state = read_state_value(&repo_root).expect("state should be readable");
        assert_eq!(
            updated_state
                .pointer("/review_agent/chronic_category_responses/entries/0/verification_cycle"),
            Some(&json!(461))
        );
        let commit_subject = std::process::Command::new("git")
            .arg("-C")
            .arg(&repo_root)
            .args(["log", "-1", "--pretty=%s"])
            .output()
            .expect("git log should execute");
        assert!(commit_subject.status.success(), "git log should succeed");
        let subject =
            String::from_utf8(commit_subject.stdout).expect("git log output should be UTF-8");
        assert!(subject.contains("state(process-review): chronic rollback"));
    }

    #[test]
    fn run_applies_drop_deferral_alongside_review_processing() {
        let repo_root = write_temp_state_repo(json!({
            "last_cycle": {"number": 500},
            "cycle_phase": {"cycle": 500},
            "review_agent": {
                "last_review_cycle": 499,
                "history": []
            },
            "deferred_findings": [{
                "category": "worklog-accuracy",
                "deferred_cycle": 468,
                "deadline_cycle": 473,
                "resolved": false
            }],
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 499"}
                }
            }
        }));
        init_temp_git_repo(&repo_root);
        let review_dir = repo_root.join("docs/reviews");
        fs::create_dir_all(&review_dir).expect("review directory should exist");
        fs::write(
            review_dir.join("cycle-500.md"),
            r#"# Cycle 500 Review

## Findings

1. **[review-accounting] Review accounting finding**

## Complacency score

2/5
"#,
        )
        .expect("review file should be written");

        let cli = Cli::parse_from([
            "process-review",
            "--repo-root",
            repo_root.to_str().expect("repo path should be valid UTF-8"),
            "--review-file",
            "docs/reviews/cycle-500.md",
            "--review-issue",
            "900",
            "--actioned",
            "1",
            "--drop-deferral",
            "worklog-accuracy:468:awaiting Eva response",
        ]);

        run(cli).expect("combined review and deferral run should succeed");

        let updated_state = read_state_value(&repo_root).expect("state should be readable");
        assert_eq!(
            updated_state.pointer("/deferred_findings/0/dropped_rationale"),
            Some(&json!("awaiting Eva response"))
        );
        assert_eq!(
            updated_state.pointer("/deferred_findings/0/resolved"),
            Some(&json!(true))
        );
        assert_eq!(
            updated_state.pointer("/deferred_findings/0/resolved_ref"),
            Some(&json!("dropped: awaiting Eva response"))
        );
        assert_eq!(
            updated_state.pointer("/review_agent/history/0/cycle"),
            Some(&json!(500))
        );
        assert_eq!(
            updated_state.pointer("/review_agent/history/0/review_issue"),
            Some(&json!(900))
        );
    }

    #[test]
    fn parse_review_accepts_category_line_tags_by_default() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let markdown = r#"## Findings

1. **Finding title**
Category: Review Accounting

## Complacency score

**2/5**
"#;
        let parsed = parse_review(path, markdown, false).expect("parse should succeed");
        assert_eq!(parsed.categories, vec!["review-accounting".to_string()]);
    }

    #[test]
    fn parse_review_accepts_findings_without_findings_header() {
        let path = Path::new("docs/reviews/cycle-213.md");
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] First finding

## 2. [tooling-contract] Second finding

## Complacency score

**4/5**
"#;

        let parsed = parse_review(path, markdown, false).expect("parse should succeed");
        assert_eq!(parsed.cycle, 213);
        assert_eq!(parsed.finding_count, 2);
        assert_eq!(parsed.complacency_score, 4);
        assert_eq!(
            parsed.categories,
            vec![
                "review-evidence".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn parse_review_rejects_missing_category_by_default() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let error = parse_review(path, SAMPLE_REVIEW, false).expect_err("parse should fail");
        assert!(error.contains("Finding 3 has no category tag"));
    }

    #[test]
    fn parse_review_accepts_cycle_197_artifact() {
        let path = Path::new("docs/reviews/cycle-197.md");
        let parsed = parse_review(path, CYCLE_197_REVIEW, false).expect("parse should succeed");
        assert_eq!(parsed.cycle, 197);
        assert_eq!(parsed.complacency_score, 5);
        assert_eq!(parsed.finding_count, 4);
    }

    #[test]
    fn parse_review_accepts_cycle_205_artifact() {
        let path = Path::new("docs/reviews/cycle-205.md");
        let parsed = parse_review(path, CYCLE_205_REVIEW, false).expect("parse should succeed");
        assert_eq!(parsed.cycle, 205);
        assert_eq!(parsed.complacency_score, 4);
        assert_eq!(parsed.finding_count, 4);
        assert_eq!(
            parsed.categories,
            vec![
                "complacency-audit".to_string(),
                "state-integrity".to_string(),
                "tooling-contract".to_string(),
                "worklog-accuracy".to_string(),
            ]
        );
    }

    #[test]
    fn parse_review_lenient_mode_accepts_legacy_category_lines() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let parsed = parse_review(path, SAMPLE_REVIEW, true).expect("parse should succeed");
        assert_eq!(
            parsed.categories,
            vec![
                "data-integrity".to_string(),
                "process-integrity".to_string()
            ]
        );
    }

    #[test]
    fn review_format_validation_rejects_missing_finding_headings() {
        let markdown = r#"## Findings

No numbered findings here.

## Complacency score

**2/5**
"#;

        assert_eq!(
            validate_review_format(markdown),
            vec![
                "review markdown must contain at least one numbered finding heading in the Findings section"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn review_format_validation_rejects_missing_complacency_score_section() {
        let markdown = r#"## Findings

1. **[tooling-contract] Finding title**
"#;

        assert_eq!(
            validate_review_format(markdown),
            vec![
                "review markdown must contain a complacency score section with a parsable N/5 score"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn review_format_validation_accepts_hash_prefixed_findings() {
        let markdown = r#"## Findings

## 1. [worklog-accuracy] Finding title

## Complacency score

4/5 — actual score
"#;

        assert!(validate_review_format(markdown).is_empty());
    }

    #[test]
    fn review_format_validation_accepts_category_lines() {
        let markdown = r#"## Findings

1. **Finding title**
Category: Review Accounting

## Complacency score

4/5 — actual score
"#;

        assert!(validate_review_format(markdown).is_empty());
    }

    #[test]
    fn review_format_validation_accepts_findings_without_findings_header() {
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] Finding title

## Complacency score

4/5 — actual score
"#;

        assert!(validate_review_format(markdown).is_empty());
    }

    fn write_temp_state_repo(state: Value) -> PathBuf {
        use std::time::{SystemTime, UNIX_EPOCH};

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let repo_root = std::env::temp_dir().join(format!(
            "process-review-test-{}-{}",
            std::process::id(),
            unique
        ));
        let docs_dir = repo_root.join("docs");
        fs::create_dir_all(&docs_dir).expect("temp docs directory should be created");
        fs::write(
            docs_dir.join("state.json"),
            serde_json::to_string(&state).expect("state json should serialize"),
        )
        .expect("state.json should be written");
        repo_root
    }

    fn chronic_state_fixture(entries: Value) -> Value {
        json!({
            "last_cycle": {"number": 500},
            "cycle_phase": {"cycle": 500},
            "review_agent": {
                "last_review_cycle": 499,
                "history": [],
                "chronic_category_responses": {
                    "entries": entries
                }
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 499"},
                    "review_agent.chronic_category_responses": {"last_refreshed": "cycle 448"}
                }
            }
        })
    }

    fn init_temp_git_repo(repo_root: &Path) {
        let status = std::process::Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .arg("init")
            .status()
            .expect("git init should execute");
        assert!(status.success(), "git init should succeed");

        let email_status = std::process::Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(["config", "user.email", "copilot@example.com"])
            .status()
            .expect("git config user.email should execute");
        assert!(
            email_status.success(),
            "git config user.email should succeed"
        );

        let name_status = std::process::Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(["config", "user.name", "Copilot"])
            .status()
            .expect("git config user.name should execute");
        assert!(name_status.success(), "git config user.name should succeed");
    }
}
