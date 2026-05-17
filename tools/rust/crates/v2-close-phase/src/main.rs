use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const SCHEMA: &str = "v2-close-phase/v1";

#[derive(Parser, Debug)]
#[command(
    name = "v2-close-phase",
    about = "Orchestration-hub for v2 close-phase: pre-check git-safety, gardening-sweep, cycle-history append, journal commit+push, cycle-issue close, receipt-validate. \
             COMPLETE: receipt-validate pass (state pointer + push confirmation + issue state) + idempotent re-run + git-safety pre-check landed cycle 137. \
             DEFERRED still: multi-cycle batch close, rollback semantics, Eva-response carry-forward, smart commit-message generation, partial-success rollback, concurrent-runner detection, journal-immutability enforcement."
)]
struct Args {
    /// Cycle number being closed (required)
    #[arg(long)]
    cycle_n: u64,

    /// Repository root (path containing the state directory + journal directory)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Path of the cycle-history directory relative to repo_root
    #[arg(long, default_value = "state/cycle-history")]
    state_dir: PathBuf,

    /// Path to today's journal file relative to repo_root (e.g. docs/journal/2026-05-13.md)
    #[arg(long)]
    journal_path: Option<PathBuf>,

    /// GitHub issue number for this cycle (used by issue-close stage; required unless --skip-issue-close)
    #[arg(long)]
    issue_number: Option<u64>,

    /// Repository slug owner/name for gh shell-outs. When unset, gh uses cwd remote.
    #[arg(long)]
    repo_slug: Option<String>,

    /// JSON payload (path) for cycle-history append. See v2-cycle-history-append --from-json.
    #[arg(long)]
    history_payload: Option<PathBuf>,

    /// Path to a body file for the issue-close comment (the session-end summary).
    #[arg(long)]
    close_comment_body: Option<PathBuf>,

    /// Commit message for the journal commit. If unset, a default is generated.
    #[arg(long)]
    commit_message: Option<String>,

    /// Gardening-sweep corpus paths (repeatable). Forwarded to v2-gardening-sweep --corpus.
    #[arg(long = "gardening-corpus")]
    gardening_corpus: Vec<PathBuf>,

    /// Gardening-sweep exclude glob patterns (repeatable).
    #[arg(long = "gardening-exclude")]
    gardening_exclude: Vec<String>,

    /// Stage skip flags. When set, the named stage is recorded as Skipped (not run).
    #[arg(long)]
    skip_gardening: bool,

    #[arg(long)]
    skip_history_append: bool,

    #[arg(long)]
    skip_git_push: bool,

    #[arg(long)]
    skip_issue_close: bool,

    /// Skip the git-safety pre-check stage. Use only when the caller has
    /// already verified there are no unpushed local commits — e.g. in a
    /// hermetic test environment or when running the very first cycle on
    /// a freshly-cloned repo with no upstream tracking.
    #[arg(long)]
    skip_pre_check: bool,

    /// Skip the receipt-validate stage. The validation pass is the
    /// COMPLETE-grade differentiator from SCAFFOLD-PARTIAL; skipping it
    /// reverts to the scaffold-honesty stance where the receipt only
    /// records captured artifacts.
    #[arg(long)]
    skip_receipt_validate: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// Write output to a file ("-" for stdout, default).
    #[arg(long, default_value = "-")]
    output: String,

    /// Strict mode — promote Warn/Deferred stages to non-zero exit.
    #[arg(long)]
    strict: bool,

    /// Dry-run mode — log subprocess invocations but do not execute side-effects
    /// (no gardening-sweep run, no history-append write, no git commit/push, no
    /// gh comment/close). Useful for "what would this cycle close do" inspection.
    #[arg(long)]
    dry_run: bool,

    /// Read subprocess outputs from this directory instead of shelling out.
    /// Expected files (any missing file makes the corresponding stage Failed):
    ///   - gardening-sweep.json (v2-gardening-sweep JSON output)
    ///   - cycle-history-append.txt (the path that would be written)
    ///   - git-commit-sha.txt (the commit SHA the push would advance to)
    ///   - issue-comment-url.txt (the URL of the close-comment that would be posted)
    ///
    /// Implies --dry-run for the side-effect commands; the receipt scaffold still
    /// runs against the fixture contents.
    #[arg(long)]
    fixture_dir: Option<PathBuf>,

    /// Path to the v2-gardening-sweep binary (default: looks up on PATH or via cargo target dir).
    #[arg(long, default_value = "v2-gardening-sweep")]
    gardening_sweep_bin: String,

    /// Path to the v2-cycle-history-append binary.
    #[arg(long, default_value = "v2-cycle-history-append")]
    history_append_bin: String,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum StageStatus {
    Done,
    Warn,
    /// Preserved as scaffold-honesty primitive (cycle 123 NOVEL@1, cycle 124
    /// HARDENED-via-completion). Strict mode rejects it. No built-in stage
    /// emits Deferred after cycle 137 COMPLETE removed the receipt-scaffold
    /// always-Deferred stub, but the variant is retained so future scaffold
    /// stages can use it without re-introducing the primitive.
    #[allow(dead_code)]
    Deferred,
    Failed,
    Skipped,
}

#[derive(Debug, Serialize)]
struct StageResult {
    name: String,
    status: StageStatus,
    details: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Value>,
}

#[derive(Debug, Serialize)]
struct CloseReport {
    schema: &'static str,
    cycle_n: u64,
    dry_run: bool,
    fixture_mode: bool,
    summary: StageSummary,
    stages: Vec<StageResult>,
    /// SCAFFOLD-PARTIAL: receipt validation against state.json is deferred.
    /// The receipt scaffold below captures the artifacts that would be
    /// reconciled by a future receipt-validate stage.
    receipt: ReceiptScaffold,
}

#[derive(Debug, Serialize, Default)]
struct StageSummary {
    done: u32,
    warn: u32,
    deferred: u32,
    failed: u32,
    skipped: u32,
}

impl StageSummary {
    fn from_stages(stages: &[StageResult]) -> Self {
        let mut s = StageSummary::default();
        for stage in stages {
            match stage.status {
                StageStatus::Done => s.done += 1,
                StageStatus::Warn => s.warn += 1,
                StageStatus::Deferred => s.deferred += 1,
                StageStatus::Failed => s.failed += 1,
                StageStatus::Skipped => s.skipped += 1,
            }
        }
        s
    }

    fn worst_exit_code(&self, strict: bool) -> u8 {
        if self.failed > 0 {
            return 2;
        }
        if strict && (self.warn > 0 || self.deferred > 0) {
            return 1;
        }
        0
    }
}

/// Receipt of what close-phase produced this cycle, with validation outcomes
/// from `run_receipt_validate` populated when --skip-receipt-validate is not set.
/// Field names preserved from cycle 136 SCAFFOLD-PARTIAL (`state_pointer_check`
/// and `push_confirmation` were declared but not populated then); cycle 137
/// COMPLETE populates them and adds `issue_state_check`.
#[derive(Debug, Serialize, Default)]
struct ReceiptScaffold {
    gardening_findings_count: Option<u64>,
    cycle_history_path: Option<String>,
    commit_sha: Option<String>,
    issue_comment_url: Option<String>,
    /// COMPLETE (cycle 137): "ok: <detail>" if the cycle-history file exists
    /// on disk; "err: <detail>" if not; None if no path was captured.
    state_pointer_check: Option<String>,
    /// COMPLETE (cycle 137): "ok: <detail>" if `git ls-remote` confirms the
    /// commit reached the remote (exact match or ancestor of remote HEAD);
    /// "err: <detail>" if not; "skipped: <reason>" in dry-run/fixture; None
    /// if no commit SHA was captured.
    push_confirmation: Option<String>,
    /// COMPLETE (cycle 137): "ok: closed" if `gh issue view` confirms the
    /// issue state == CLOSED; "err: <state>" if not; "skipped: <reason>" in
    /// dry-run/fixture; None if no issue number was provided.
    issue_state_check: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("v2-close-phase: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &Args) -> Result<u8, String> {
    let fixture_mode = args.fixture_dir.is_some();
    let effective_dry_run = args.dry_run || fixture_mode;

    let mut stages: Vec<StageResult> = Vec::new();
    let mut receipt = ReceiptScaffold::default();

    // Stage 0: git-safety pre-check (cycle 137 COMPLETE — preserves cycle 524 lesson)
    if args.skip_pre_check {
        stages.push(StageResult {
            name: "pre-check".to_string(),
            status: StageStatus::Skipped,
            details: "--skip-pre-check was set".to_string(),
            output: None,
        });
    } else {
        let stage = pre_check_git_safety(args, effective_dry_run);
        let abort = matches!(stage.status, StageStatus::Failed);
        stages.push(stage);
        if abort {
            // git-safety violation aborts the pipeline — do not stack destructive
            // work on top of an inconsistent repo state. The Failed pre-check
            // becomes the only non-skipped stage; remaining stages are recorded
            // as Skipped with the pre-check abort reason.
            for name in [
                "gardening-sweep",
                "cycle-history-append",
                "journal-commit-push",
                "issue-close",
                "receipt-validate",
            ] {
                stages.push(StageResult {
                    name: name.to_string(),
                    status: StageStatus::Skipped,
                    details: "aborted: pre-check git-safety violation".to_string(),
                    output: None,
                });
            }
            let summary = StageSummary::from_stages(&stages);
            let report = CloseReport {
                schema: SCHEMA,
                cycle_n: args.cycle_n,
                dry_run: effective_dry_run,
                fixture_mode,
                summary: StageSummary::from_stages(&stages),
                stages,
                receipt,
            };
            write_output(args, &report)?;
            return Ok(summary.worst_exit_code(args.strict));
        }
    }

    // Stage 1: gardening-sweep
    if args.skip_gardening {
        stages.push(StageResult {
            name: "gardening-sweep".to_string(),
            status: StageStatus::Skipped,
            details: "--skip-gardening was set".to_string(),
            output: None,
        });
    } else {
        let stage = run_gardening_sweep(args, effective_dry_run, &mut receipt);
        stages.push(stage);
    }

    // Stage 2: cycle-history append
    if args.skip_history_append {
        stages.push(StageResult {
            name: "cycle-history-append".to_string(),
            status: StageStatus::Skipped,
            details: "--skip-history-append was set".to_string(),
            output: None,
        });
    } else {
        let stage = run_history_append(args, effective_dry_run, &mut receipt);
        stages.push(stage);
    }

    // Stage 3: journal commit + push (the load-bearing git-safety primitive — push WITH commit)
    if args.skip_git_push {
        stages.push(StageResult {
            name: "journal-commit-push".to_string(),
            status: StageStatus::Skipped,
            details: "--skip-git-push was set".to_string(),
            output: None,
        });
    } else {
        let stage = run_journal_commit_push(args, effective_dry_run, &mut receipt);
        stages.push(stage);
    }

    // Stage 4: issue-close (comment + close)
    if args.skip_issue_close {
        stages.push(StageResult {
            name: "issue-close".to_string(),
            status: StageStatus::Skipped,
            details: "--skip-issue-close was set".to_string(),
            output: None,
        });
    } else {
        let stage = run_issue_close(args, effective_dry_run, &mut receipt);
        stages.push(stage);
    }

    // Stage 5: receipt-validate (cycle 137 COMPLETE — state pointer + push confirmation + issue state)
    if args.skip_receipt_validate {
        stages.push(StageResult {
            name: "receipt-validate".to_string(),
            status: StageStatus::Skipped,
            details: "--skip-receipt-validate was set".to_string(),
            output: None,
        });
    } else {
        let stage = run_receipt_validate(args, effective_dry_run, &mut receipt);
        stages.push(stage);
    }

    let summary = StageSummary::from_stages(&stages);
    let report = CloseReport {
        schema: SCHEMA,
        cycle_n: args.cycle_n,
        dry_run: effective_dry_run,
        fixture_mode,
        summary: StageSummary::from_stages(&stages),
        stages,
        receipt,
    };

    write_output(args, &report)?;

    Ok(summary.worst_exit_code(args.strict))
}

// -------------------------------------------------------------------
// Stage 1: gardening-sweep
// -------------------------------------------------------------------

fn run_gardening_sweep(args: &Args, dry_run: bool, receipt: &mut ReceiptScaffold) -> StageResult {
    if let Some(fixture_dir) = &args.fixture_dir {
        let path = fixture_dir.join("gardening-sweep.json");
        return match fs::read_to_string(&path) {
            Ok(s) => match serde_json::from_str::<Value>(&s) {
                Ok(v) => {
                    let findings_count = count_gardening_findings(&v);
                    receipt.gardening_findings_count = Some(findings_count);
                    let status = if findings_count == 0 {
                        StageStatus::Done
                    } else {
                        StageStatus::Warn
                    };
                    StageResult {
                        name: "gardening-sweep".to_string(),
                        status,
                        details: format!("fixture: {findings_count} findings"),
                        output: Some(v),
                    }
                }
                Err(e) => StageResult {
                    name: "gardening-sweep".to_string(),
                    status: StageStatus::Failed,
                    details: format!("fixture parse error: {e}"),
                    output: None,
                },
            },
            Err(e) => StageResult {
                name: "gardening-sweep".to_string(),
                status: StageStatus::Failed,
                details: format!("fixture read error at {}: {e}", path.display()),
                output: None,
            },
        };
    }

    if args.gardening_corpus.is_empty() {
        return StageResult {
            name: "gardening-sweep".to_string(),
            status: StageStatus::Warn,
            details: "no --gardening-corpus paths set; nothing to sweep".to_string(),
            output: None,
        };
    }

    if dry_run {
        return StageResult {
            name: "gardening-sweep".to_string(),
            status: StageStatus::Skipped,
            details: format!(
                "dry-run: would invoke `{}` with {} corpus paths, {} excludes",
                args.gardening_sweep_bin,
                args.gardening_corpus.len(),
                args.gardening_exclude.len()
            ),
            output: None,
        };
    }

    let mut cmd = Command::new(&args.gardening_sweep_bin);
    cmd.current_dir(&args.repo_root);
    cmd.arg("--format").arg("json");
    cmd.arg("--output").arg("-");
    for corpus in &args.gardening_corpus {
        cmd.arg("--corpus").arg(corpus);
    }
    for exclude in &args.gardening_exclude {
        cmd.arg("--exclude").arg(exclude);
    }

    match cmd.output() {
        Ok(output) => {
            if !output.status.success() && output.status.code() != Some(0) {
                // gardening-sweep returns 0 even with findings unless --strict.
                // A non-zero from us not using --strict is a real failure.
                let stderr = String::from_utf8_lossy(&output.stderr);
                return StageResult {
                    name: "gardening-sweep".to_string(),
                    status: StageStatus::Failed,
                    details: format!(
                        "exit {}: {}",
                        output.status.code().unwrap_or(-1),
                        stderr.trim()
                    ),
                    output: None,
                };
            }
            let stdout = String::from_utf8_lossy(&output.stdout);
            match serde_json::from_str::<Value>(&stdout) {
                Ok(v) => {
                    let findings_count = count_gardening_findings(&v);
                    receipt.gardening_findings_count = Some(findings_count);
                    let status = if findings_count == 0 {
                        StageStatus::Done
                    } else {
                        StageStatus::Warn
                    };
                    StageResult {
                        name: "gardening-sweep".to_string(),
                        status,
                        details: format!("{findings_count} findings"),
                        output: Some(v),
                    }
                }
                Err(e) => StageResult {
                    name: "gardening-sweep".to_string(),
                    status: StageStatus::Failed,
                    details: format!("parse error: {e}"),
                    output: None,
                },
            }
        }
        Err(e) => StageResult {
            name: "gardening-sweep".to_string(),
            status: StageStatus::Failed,
            details: format!("exec error: {e}"),
            output: None,
        },
    }
}

fn count_gardening_findings(v: &Value) -> u64 {
    let stale = v
        .get("stale")
        .and_then(|s| s.as_array())
        .map(|a| a.len() as u64)
        .unwrap_or(0);
    let dead = v
        .get("dead_links")
        .and_then(|s| s.as_array())
        .map(|a| a.len() as u64)
        .unwrap_or(0);
    stale + dead
}

// -------------------------------------------------------------------
// Stage 2: cycle-history append
// -------------------------------------------------------------------

fn run_history_append(args: &Args, dry_run: bool, receipt: &mut ReceiptScaffold) -> StageResult {
    if let Some(fixture_dir) = &args.fixture_dir {
        let path = fixture_dir.join("cycle-history-append.txt");
        return match fs::read_to_string(&path) {
            Ok(s) => {
                let written_path = s.trim().to_string();
                receipt.cycle_history_path = Some(written_path.clone());
                StageResult {
                    name: "cycle-history-append".to_string(),
                    status: StageStatus::Done,
                    details: format!("fixture: would write {written_path}"),
                    output: None,
                }
            }
            Err(e) => StageResult {
                name: "cycle-history-append".to_string(),
                status: StageStatus::Failed,
                details: format!("fixture read error at {}: {e}", path.display()),
                output: None,
            },
        };
    }

    let payload_path = match &args.history_payload {
        Some(p) => p.clone(),
        None => {
            return StageResult {
                name: "cycle-history-append".to_string(),
                status: StageStatus::Warn,
                details: "--history-payload not set; cannot append cycle-history entry".to_string(),
                output: None,
            };
        }
    };

    if dry_run {
        return StageResult {
            name: "cycle-history-append".to_string(),
            status: StageStatus::Skipped,
            details: format!(
                "dry-run: would invoke `{}` --cycle-n {} --from-json {}",
                args.history_append_bin,
                args.cycle_n,
                payload_path.display()
            ),
            output: None,
        };
    }

    let mut cmd = Command::new(&args.history_append_bin);
    cmd.current_dir(&args.repo_root);
    cmd.arg("--cycle-n").arg(args.cycle_n.to_string());
    cmd.arg("--from-json").arg(&payload_path);
    cmd.arg("--state-dir").arg(&args.state_dir);

    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return StageResult {
                    name: "cycle-history-append".to_string(),
                    status: StageStatus::Failed,
                    details: format!(
                        "exit {}: {}",
                        output.status.code().unwrap_or(-1),
                        stderr.trim()
                    ),
                    output: None,
                };
            }
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            receipt.cycle_history_path = Some(stdout.clone());
            StageResult {
                name: "cycle-history-append".to_string(),
                status: StageStatus::Done,
                details: format!("wrote {stdout}"),
                output: None,
            }
        }
        Err(e) => StageResult {
            name: "cycle-history-append".to_string(),
            status: StageStatus::Failed,
            details: format!("exec error: {e}"),
            output: None,
        },
    }
}

// -------------------------------------------------------------------
// Stage 3: journal commit + push (push-WITH-commit per git-safety primitive)
// -------------------------------------------------------------------

fn run_journal_commit_push(
    args: &Args,
    dry_run: bool,
    receipt: &mut ReceiptScaffold,
) -> StageResult {
    if let Some(fixture_dir) = &args.fixture_dir {
        let path = fixture_dir.join("git-commit-sha.txt");
        return match fs::read_to_string(&path) {
            Ok(s) => {
                let sha = s.trim().to_string();
                receipt.commit_sha = Some(sha.clone());
                StageResult {
                    name: "journal-commit-push".to_string(),
                    status: StageStatus::Done,
                    details: format!("fixture: would advance to {sha}"),
                    output: None,
                }
            }
            Err(e) => StageResult {
                name: "journal-commit-push".to_string(),
                status: StageStatus::Failed,
                details: format!("fixture read error at {}: {e}", path.display()),
                output: None,
            },
        };
    }

    let journal_path = match &args.journal_path {
        Some(p) => p.clone(),
        None => {
            return StageResult {
                name: "journal-commit-push".to_string(),
                status: StageStatus::Warn,
                details: "--journal-path not set; cannot stage journal commit".to_string(),
                output: None,
            };
        }
    };

    if dry_run {
        return StageResult {
            name: "journal-commit-push".to_string(),
            status: StageStatus::Skipped,
            details: format!(
                "dry-run: would `git add {}` + commit + push (commit message: {})",
                journal_path.display(),
                args.commit_message
                    .clone()
                    .unwrap_or_else(|| default_commit_message(args))
            ),
            output: None,
        };
    }

    // Stage the journal (plus any cycle-history file produced this run)
    let mut to_stage: Vec<PathBuf> = vec![journal_path.clone()];
    if let Some(p) = &receipt.cycle_history_path {
        to_stage.push(PathBuf::from(p));
    }

    let mut details = String::new();

    for path in &to_stage {
        let out = Command::new("git")
            .current_dir(&args.repo_root)
            .arg("add")
            .arg(path)
            .output();
        match out {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return StageResult {
                    name: "journal-commit-push".to_string(),
                    status: StageStatus::Failed,
                    details: format!("git add {} failed: {}", path.display(), stderr.trim()),
                    output: None,
                };
            }
            Err(e) => {
                return StageResult {
                    name: "journal-commit-push".to_string(),
                    status: StageStatus::Failed,
                    details: format!("git add exec error: {e}"),
                    output: None,
                };
            }
        }
    }

    let message = args
        .commit_message
        .clone()
        .unwrap_or_else(|| default_commit_message(args));

    let commit_out = Command::new("git")
        .current_dir(&args.repo_root)
        .arg("commit")
        .arg("-m")
        .arg(&message)
        .output();

    let commit_sha = match commit_out {
        Ok(output) if output.status.success() => {
            let head = Command::new("git")
                .current_dir(&args.repo_root)
                .arg("rev-parse")
                .arg("HEAD")
                .output()
                .map_err(|e| format!("git rev-parse exec error: {e}"))
                .and_then(|o| {
                    if o.status.success() {
                        Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
                    } else {
                        Err(format!(
                            "git rev-parse exit {}",
                            o.status.code().unwrap_or(-1)
                        ))
                    }
                });
            match head {
                Ok(s) => s,
                Err(e) => {
                    return StageResult {
                        name: "journal-commit-push".to_string(),
                        status: StageStatus::Failed,
                        details: e,
                        output: None,
                    };
                }
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            // "nothing to commit, working tree clean" is a Warn — close-phase is idempotent enough
            // that no journal change is tolerated; the caller may have already committed.
            if stdout.contains("nothing to commit") || stderr.contains("nothing to commit") {
                return StageResult {
                    name: "journal-commit-push".to_string(),
                    status: StageStatus::Warn,
                    details: "nothing to commit (journal already committed?)".to_string(),
                    output: None,
                };
            }
            return StageResult {
                name: "journal-commit-push".to_string(),
                status: StageStatus::Failed,
                details: format!(
                    "git commit exit {}: {} / {}",
                    output.status.code().unwrap_or(-1),
                    stdout.trim(),
                    stderr.trim()
                ),
                output: None,
            };
        }
        Err(e) => {
            return StageResult {
                name: "journal-commit-push".to_string(),
                status: StageStatus::Failed,
                details: format!("git commit exec error: {e}"),
                output: None,
            };
        }
    };

    // Push — the load-bearing primitive per git-safety: push with the same commit.
    let push_out = Command::new("git")
        .current_dir(&args.repo_root)
        .arg("push")
        .output();

    match push_out {
        Ok(output) if output.status.success() => {
            receipt.commit_sha = Some(commit_sha.clone());
            details.push_str(&format!("committed + pushed {}", short_sha(&commit_sha)));
            StageResult {
                name: "journal-commit-push".to_string(),
                status: StageStatus::Done,
                details,
                output: None,
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Commit succeeded; push failed. The git-safety primitive treats this as Failed;
            // the next cycle's boot-phase will see the unpushed local commit and surface a
            // git-safety violation.
            StageResult {
                name: "journal-commit-push".to_string(),
                status: StageStatus::Failed,
                details: format!(
                    "commit {} created but push failed: {}",
                    short_sha(&commit_sha),
                    stderr.trim()
                ),
                output: None,
            }
        }
        Err(e) => StageResult {
            name: "journal-commit-push".to_string(),
            status: StageStatus::Failed,
            details: format!(
                "commit {} created but push exec error: {e}",
                short_sha(&commit_sha)
            ),
            output: None,
        },
    }
}

fn default_commit_message(args: &Args) -> String {
    format!(
        "redesign(phase-2): cycle {} — journal entry + cycle-history append",
        args.cycle_n
    )
}

fn short_sha(sha: &str) -> String {
    sha.chars().take(8).collect()
}

// -------------------------------------------------------------------
// Stage 4: issue-close (comment + close)
// -------------------------------------------------------------------

fn run_issue_close(args: &Args, dry_run: bool, receipt: &mut ReceiptScaffold) -> StageResult {
    if let Some(fixture_dir) = &args.fixture_dir {
        let path = fixture_dir.join("issue-comment-url.txt");
        return match fs::read_to_string(&path) {
            Ok(s) => {
                let url = s.trim().to_string();
                receipt.issue_comment_url = Some(url.clone());
                StageResult {
                    name: "issue-close".to_string(),
                    status: StageStatus::Done,
                    details: format!("fixture: would post + close at {url}"),
                    output: None,
                }
            }
            Err(e) => StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Failed,
                details: format!("fixture read error at {}: {e}", path.display()),
                output: None,
            },
        };
    }

    let issue_number = match args.issue_number {
        Some(n) => n,
        None => {
            return StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Warn,
                details: "--issue-number not set; cannot close cycle-issue".to_string(),
                output: None,
            };
        }
    };

    let body_path = match &args.close_comment_body {
        Some(p) => p.clone(),
        None => {
            return StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Warn,
                details: "--close-comment-body not set; cannot post close comment".to_string(),
                output: None,
            };
        }
    };

    if dry_run {
        return StageResult {
            name: "issue-close".to_string(),
            status: StageStatus::Skipped,
            details: format!(
                "dry-run: would `gh issue comment {} --body-file {}` + `gh issue close {}`",
                issue_number,
                body_path.display(),
                issue_number
            ),
            output: None,
        };
    }

    // Idempotency check (cycle 137 COMPLETE): if the issue is already closed,
    // skip the comment-and-close to avoid duplicate close-comments on re-runs.
    // Failures of the state query are non-fatal — proceed with comment+close
    // and let those operations surface any auth/network issues themselves.
    if let Some(state) = fetch_issue_state(args, issue_number) {
        if state.eq_ignore_ascii_case("closed") {
            receipt.issue_comment_url = None;
            return StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Done,
                details: format!(
                    "issue {issue_number} already closed (idempotent re-run; comment not posted)"
                ),
                output: None,
            };
        }
    }

    let mut comment_cmd = Command::new("gh");
    comment_cmd.current_dir(&args.repo_root);
    comment_cmd
        .arg("issue")
        .arg("comment")
        .arg(issue_number.to_string());
    comment_cmd.arg("--body-file").arg(&body_path);
    if let Some(slug) = &args.repo_slug {
        comment_cmd.arg("--repo").arg(slug);
    }

    let comment_out = comment_cmd.output();
    let comment_url = match comment_out {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Failed,
                details: format!(
                    "gh issue comment exit {}: {}",
                    output.status.code().unwrap_or(-1),
                    stderr.trim()
                ),
                output: None,
            };
        }
        Err(e) => {
            return StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Failed,
                details: format!("gh issue comment exec error: {e}"),
                output: None,
            };
        }
    };

    let mut close_cmd = Command::new("gh");
    close_cmd.current_dir(&args.repo_root);
    close_cmd
        .arg("issue")
        .arg("close")
        .arg(issue_number.to_string());
    if let Some(slug) = &args.repo_slug {
        close_cmd.arg("--repo").arg(slug);
    }

    let close_out = close_cmd.output();
    match close_out {
        Ok(output) if output.status.success() => {
            receipt.issue_comment_url = Some(comment_url.clone());
            StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Done,
                details: format!("commented + closed; comment {comment_url}"),
                output: None,
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Comment succeeded; close failed. Partial state — surface as Failed so
            // the next cycle's boot-phase sees the still-open issue and recovers.
            StageResult {
                name: "issue-close".to_string(),
                status: StageStatus::Failed,
                details: format!(
                    "comment {} posted but issue close failed: {}",
                    comment_url,
                    stderr.trim()
                ),
                output: None,
            }
        }
        Err(e) => StageResult {
            name: "issue-close".to_string(),
            status: StageStatus::Failed,
            details: format!(
                "comment {} posted but issue close exec error: {e}",
                comment_url
            ),
            output: None,
        },
    }
}

// -------------------------------------------------------------------
// Stage 0: git-safety pre-check (cycle 137 COMPLETE)
// -------------------------------------------------------------------

/// Pre-check that the repo is in a state where close-phase can run safely.
/// The dominant concern is the cycle 524 corruption class: an unpushed local
/// commit from a prior cycle would silently turn close-phase's commit+push
/// into a stack of two pushed commits, breaking the "every commit is pushed
/// in the same operation" git-safety primitive (see PRESERVED-PRIMITIVES).
///
/// In dry-run / fixture mode, the pre-check is skipped — those modes do not
/// touch the working tree.
fn pre_check_git_safety(args: &Args, dry_run: bool) -> StageResult {
    if dry_run {
        return StageResult {
            name: "pre-check".to_string(),
            status: StageStatus::Skipped,
            details: "dry-run: no unpushed-commits check".to_string(),
            output: None,
        };
    }

    let out = Command::new("git")
        .current_dir(&args.repo_root)
        .arg("rev-list")
        .arg("--count")
        .arg("@{u}..HEAD")
        .output();

    match out {
        Ok(output) if output.status.success() => {
            let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let count: u64 = count_str.parse().unwrap_or(0);
            if count == 0 {
                StageResult {
                    name: "pre-check".to_string(),
                    status: StageStatus::Done,
                    details: "no unpushed local commits".to_string(),
                    output: None,
                }
            } else {
                StageResult {
                    name: "pre-check".to_string(),
                    status: StageStatus::Failed,
                    details: format!(
                        "{count} unpushed local commit(s) detected; cycle 524 git-safety violation — resolve before close-phase. Run `git push` to publish or `git reset --soft @{{u}}` to discard."
                    ),
                    output: None,
                }
            }
        }
        Ok(output) => {
            // Most common cause: no upstream tracking branch configured.
            // Surface as Warn (not Failed) so first-cycle / detached-HEAD
            // setups don't get stuck. Caller can pass --skip-pre-check to
            // suppress entirely.
            let stderr = String::from_utf8_lossy(&output.stderr);
            StageResult {
                name: "pre-check".to_string(),
                status: StageStatus::Warn,
                details: format!(
                    "git rev-list @{{u}}..HEAD non-zero exit: {} (no upstream tracking branch?)",
                    stderr.trim()
                ),
                output: None,
            }
        }
        Err(e) => StageResult {
            name: "pre-check".to_string(),
            status: StageStatus::Failed,
            details: format!("git rev-list exec error: {e}"),
            output: None,
        },
    }
}

// -------------------------------------------------------------------
// Stage 5: receipt-validate (cycle 137 COMPLETE)
// -------------------------------------------------------------------

/// Validate the artifacts captured in the receipt actually landed.
///
/// Three independent checks:
///   - state-pointer: the cycle-history file exists on disk at the captured path
///   - push-confirmation: `git ls-remote origin HEAD` resolves to a commit
///     that either equals the captured commit_sha or has it as an ancestor
///   - issue-state: `gh issue view N --json state` returns CLOSED
///
/// When the corresponding artifact wasn't captured (e.g. the stage was skipped
/// or produced no SHA), the check is recorded as "skipped: <reason>" and the
/// receipt field is left None. A check counts as failed only when an artifact
/// WAS captured and validation rejected it.
fn run_receipt_validate(args: &Args, dry_run: bool, receipt: &mut ReceiptScaffold) -> StageResult {
    let mut checks: Vec<(&'static str, bool, String)> = Vec::new();

    // Check 1: state-pointer
    let state_pointer_detail = match &receipt.cycle_history_path {
        Some(path) => {
            if dry_run {
                ("skipped: dry-run".to_string(), true)
            } else {
                let full = args.repo_root.join(path);
                if full.exists() {
                    (format!("ok: {path}"), true)
                } else {
                    (format!("err: {path} not on disk"), false)
                }
            }
        }
        None => ("skipped: no cycle_history_path captured".to_string(), true),
    };
    receipt.state_pointer_check = Some(state_pointer_detail.0.clone());
    checks.push((
        "state-pointer",
        state_pointer_detail.1,
        state_pointer_detail.0,
    ));

    // Check 2: push-confirmation
    let push_detail = match &receipt.commit_sha {
        Some(sha) => {
            if dry_run {
                ("skipped: dry-run".to_string(), true)
            } else {
                let conf = confirm_push(args, sha);
                let ok = conf.starts_with("ok:");
                (conf, ok)
            }
        }
        None => ("skipped: no commit_sha captured".to_string(), true),
    };
    receipt.push_confirmation = Some(push_detail.0.clone());
    checks.push(("push-confirmation", push_detail.1, push_detail.0));

    // Check 3: issue-state
    let issue_detail = match args.issue_number {
        Some(n) if !args.skip_issue_close => {
            if dry_run {
                ("skipped: dry-run".to_string(), true)
            } else {
                match fetch_issue_state(args, n) {
                    Some(state) if state.eq_ignore_ascii_case("closed") => {
                        ("ok: closed".to_string(), true)
                    }
                    Some(state) => (
                        format!("err: issue {n} state={state} (expected CLOSED)"),
                        false,
                    ),
                    None => (format!("err: could not fetch state for issue {n}"), false),
                }
            }
        }
        Some(_) => ("skipped: --skip-issue-close was set".to_string(), true),
        None => ("skipped: no --issue-number provided".to_string(), true),
    };
    receipt.issue_state_check = Some(issue_detail.0.clone());
    checks.push(("issue-state", issue_detail.1, issue_detail.0));

    let failed_count = checks.iter().filter(|(_, ok, _)| !*ok).count();
    let real_checks = checks
        .iter()
        .filter(|(_, _, msg)| !msg.starts_with("skipped:"))
        .count();
    let details = checks
        .iter()
        .map(|(name, ok, detail)| {
            let marker = if *ok { "OK" } else { "FAIL" };
            format!("{name}[{marker}]: {detail}")
        })
        .collect::<Vec<_>>()
        .join("; ");

    let status = if failed_count > 0 {
        StageStatus::Failed
    } else if real_checks == 0 {
        // Nothing was actually validated (no artifacts captured, or dry-run, or
        // fixture mode). Reporting Done would over-claim — Skipped is honest.
        StageStatus::Skipped
    } else {
        StageStatus::Done
    };

    StageResult {
        name: "receipt-validate".to_string(),
        status,
        details,
        output: None,
    }
}

/// Confirm that `sha` is reachable from the remote default-branch HEAD.
/// Returns "ok: <detail>" or "err: <detail>".
fn confirm_push(args: &Args, sha: &str) -> String {
    let ls = Command::new("git")
        .current_dir(&args.repo_root)
        .arg("ls-remote")
        .arg("origin")
        .arg("HEAD")
        .output();

    let remote_head = match ls {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            match s.split_whitespace().next() {
                Some(h) => h.to_string(),
                None => return "err: empty ls-remote output for origin HEAD".to_string(),
            }
        }
        Ok(o) => {
            return format!(
                "err: git ls-remote exit {}: {}",
                o.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&o.stderr).trim()
            );
        }
        Err(e) => return format!("err: git ls-remote exec: {e}"),
    };

    if remote_head == sha {
        return format!("ok: remote HEAD == {}", short_sha(&remote_head));
    }

    // Check ancestry — if our commit is an ancestor of remote HEAD, the push
    // succeeded and a subsequent push by another writer advanced HEAD further.
    let ancestor = Command::new("git")
        .current_dir(&args.repo_root)
        .arg("merge-base")
        .arg("--is-ancestor")
        .arg(sha)
        .arg(&remote_head)
        .status();
    match ancestor {
        Ok(s) if s.success() => format!(
            "ok: {} is ancestor of remote HEAD {}",
            short_sha(sha),
            short_sha(&remote_head)
        ),
        Ok(_) => format!(
            "err: {} not reachable from remote HEAD {}",
            short_sha(sha),
            short_sha(&remote_head)
        ),
        Err(e) => format!("err: git merge-base exec: {e}"),
    }
}

/// Fetch the GitHub issue state via `gh issue view N --json state`.
/// Returns Some("open"|"closed"|...) on success, None on any error.
/// Used by both the issue-close idempotency check and receipt-validate.
fn fetch_issue_state(args: &Args, issue_number: u64) -> Option<String> {
    let mut cmd = Command::new("gh");
    cmd.current_dir(&args.repo_root);
    cmd.arg("issue").arg("view").arg(issue_number.to_string());
    cmd.arg("--json").arg("state");
    if let Some(slug) = &args.repo_slug {
        cmd.arg("--repo").arg(slug);
    }
    let out = cmd.output().ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: Value = serde_json::from_str(&stdout).ok()?;
    v.get("state")
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
}

// -------------------------------------------------------------------
// Output
// -------------------------------------------------------------------

fn write_output(args: &Args, report: &CloseReport) -> Result<(), String> {
    let serialized = match args.format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(report).map_err(|e| format!("serialization error: {e}"))?
        }
        OutputFormat::Text => render_text(report),
    };

    if args.output == "-" {
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(serialized.as_bytes())
            .map_err(|e| format!("stdout write error: {e}"))?;
        writeln!(stdout).map_err(|e| format!("stdout write error: {e}"))?;
    } else {
        let path = Path::new(&args.output);
        fs::write(path, serialized)
            .map_err(|e| format!("write error at {}: {e}", path.display()))?;
    }
    Ok(())
}

fn render_text(report: &CloseReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "v2-close-phase cycle {} ({}{})\n",
        report.cycle_n,
        if report.dry_run { "dry-run" } else { "live" },
        if report.fixture_mode { ", fixture" } else { "" }
    ));
    out.push_str(&format!(
        "  done={} warn={} deferred={} failed={} skipped={}\n",
        report.summary.done,
        report.summary.warn,
        report.summary.deferred,
        report.summary.failed,
        report.summary.skipped
    ));
    for stage in &report.stages {
        let label = match stage.status {
            StageStatus::Done => "DONE",
            StageStatus::Warn => "WARN",
            StageStatus::Deferred => "DEFER",
            StageStatus::Failed => "FAIL",
            StageStatus::Skipped => "SKIP",
        };
        out.push_str(&format!(
            "  [{label:5}] {} — {}\n",
            stage.name, stage.details
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_summary_counts() {
        let stages = vec![
            StageResult {
                name: "a".to_string(),
                status: StageStatus::Done,
                details: String::new(),
                output: None,
            },
            StageResult {
                name: "b".to_string(),
                status: StageStatus::Warn,
                details: String::new(),
                output: None,
            },
            StageResult {
                name: "c".to_string(),
                status: StageStatus::Deferred,
                details: String::new(),
                output: None,
            },
        ];
        let s = StageSummary::from_stages(&stages);
        assert_eq!(s.done, 1);
        assert_eq!(s.warn, 1);
        assert_eq!(s.deferred, 1);
        assert_eq!(s.failed, 0);
        assert_eq!(s.skipped, 0);
    }

    #[test]
    fn worst_exit_code_failed_overrides_strict() {
        let s = StageSummary {
            done: 1,
            warn: 1,
            deferred: 1,
            failed: 1,
            skipped: 0,
        };
        assert_eq!(s.worst_exit_code(false), 2);
        assert_eq!(s.worst_exit_code(true), 2);
    }

    #[test]
    fn worst_exit_code_strict_promotes_warn() {
        let s = StageSummary {
            done: 1,
            warn: 1,
            deferred: 0,
            failed: 0,
            skipped: 0,
        };
        assert_eq!(s.worst_exit_code(false), 0);
        assert_eq!(s.worst_exit_code(true), 1);
    }

    #[test]
    fn worst_exit_code_strict_promotes_deferred() {
        let s = StageSummary {
            done: 1,
            warn: 0,
            deferred: 1,
            failed: 0,
            skipped: 0,
        };
        assert_eq!(s.worst_exit_code(false), 0);
        assert_eq!(s.worst_exit_code(true), 1);
    }

    #[test]
    fn worst_exit_code_skipped_alone_is_zero() {
        let s = StageSummary {
            done: 0,
            warn: 0,
            deferred: 0,
            failed: 0,
            skipped: 4,
        };
        assert_eq!(s.worst_exit_code(false), 0);
        assert_eq!(s.worst_exit_code(true), 0);
    }

    #[test]
    fn short_sha_truncates() {
        assert_eq!(short_sha("abcdef0123456789"), "abcdef01");
        assert_eq!(short_sha("abc"), "abc");
    }

    #[test]
    fn default_commit_message_includes_cycle() {
        let args = Args {
            cycle_n: 137,
            repo_root: PathBuf::from("."),
            state_dir: PathBuf::from("state/cycle-history"),
            journal_path: None,
            issue_number: None,
            repo_slug: None,
            history_payload: None,
            close_comment_body: None,
            commit_message: None,
            gardening_corpus: vec![],
            gardening_exclude: vec![],
            skip_gardening: false,
            skip_history_append: false,
            skip_git_push: false,
            skip_issue_close: false,
            skip_pre_check: false,
            skip_receipt_validate: false,
            format: OutputFormat::Json,
            output: "-".to_string(),
            strict: false,
            dry_run: false,
            fixture_dir: None,
            gardening_sweep_bin: "v2-gardening-sweep".to_string(),
            history_append_bin: "v2-cycle-history-append".to_string(),
        };
        let msg = default_commit_message(&args);
        assert!(msg.contains("cycle 137"));
    }

    #[test]
    fn short_sha_handles_short_input() {
        assert_eq!(short_sha(""), "");
        assert_eq!(short_sha("abc"), "abc");
        assert_eq!(short_sha("abcdef01"), "abcdef01");
        assert_eq!(short_sha("abcdef012"), "abcdef01");
    }

    #[test]
    fn count_gardening_findings_handles_missing_fields() {
        let v: Value = serde_json::from_str("{}").unwrap();
        assert_eq!(count_gardening_findings(&v), 0);
        let v: Value = serde_json::from_str(r#"{"stale":[1,2]}"#).unwrap();
        assert_eq!(count_gardening_findings(&v), 2);
        let v: Value = serde_json::from_str(r#"{"stale":[1],"dead_links":[1,2,3]}"#).unwrap();
        assert_eq!(count_gardening_findings(&v), 4);
    }
}
