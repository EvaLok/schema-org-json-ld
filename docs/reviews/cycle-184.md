# Cycle 184 Review

## Findings

1. **The shared build helper fixes one stale-binary case, but it does not close the broader wrapper reliability gap**
   Category: tooling-operational-drift

   `tools/_build-helper.sh:20-30` only checks for newer `*.rs` and `Cargo.toml` files under `tools/rust/crates`. That misses workspace-level inputs such as `tools/rust/Cargo.toml`, even though the helper invokes Cargo with `--manifest-path "$tools_dir/rust/Cargo.toml"` on every rebuild. The worklog overstates the implementation as rebuilding whenever “`.rs` or `Cargo.toml` files are newer than binary” (`docs/worklog/2026-03-08/034800-hundred-eighty-fourth-orchestrator-cycle.md:29-32`), and the journal frames the fix as “structural, not instance-level” (`docs/journal/2026-03-08.md:94-96`), but the actual freshness scope is narrower than the narrative.

   The wrapper layer also still does not solve the deferred fresh-clone problem. `tools/pipeline-check:6-17` only bootstraps the `pipeline-check` binary itself; it does not ensure the downstream release binaries exist. Reproducing the issue locally with `bash tools/pipeline-check` still returns `Overall: FAIL` with `metric-snapshot`, `check-field-inventory`, `housekeeping-scan`, and `state-invariants` all skipped because their release binaries are missing. That means cycle 184 improved one wrapper failure mode, but the user-facing wrapper path requested in the review target is still broken in a fresh checkout.

2. **Receipt verification is still brittle, and the listed cycle-start receipt does not resolve**
   Category: receipt-verification-gap

   The cycle worklog explicitly says `receipt-verification-gap` was deferred (`docs/worklog/2026-03-08/034800-hundred-eighty-fourth-orchestrator-cycle.md:8`), and the cycle artifacts never compensate by recording the actual receipt hashes for the state-changing commits. There is no receipt/hash section in the worklog or journal for cycle 184, so a reviewer has to reconstruct provenance from Git history instead of reading it directly from the artifact.

   That brittleness showed up immediately: after fetching full history, `git rev-list --all | grep '^ed8b399'` returned no match, so the listed cycle-start receipt does not resolve to any commit in the repository. The actual cycle-start commit is `e661b4f12bb9b3744252bb5aed510b1d13ece8d7` (`state(cycle-start): begin cycle 184, issue #731 [cycle 184]`). Four of the five supplied receipts were real, but the missing first one proves the verification gap is still real and not just theoretical.

3. **The cycle admits the repeated `process-merge --issues` lapse, but the follow-through is weak and the backfill remains lossy**
   Category: journal-followthrough-drift

   Cycle 183 ended with a specific commitment: “When using `process-merge`, always pass `--issues` to link the agent session. Verify with `bash tools/state-invariants` after every process-merge call. If invariant 11 fails, fix immediately — don't defer.” (`docs/journal/2026-03-08.md:71-73`). Cycle 184 then admits that the very first relevant merge violated that commitment: “PR #730 was processed without `--issues 729`, requiring a backfill” (`docs/journal/2026-03-08.md:88-92`).

   The recovery was only partial. The backfilled record exists, but `docs/state.json:1818-1824` still shows issue `729` with `model: "unknown"`, so the state recovered presence/counting but not full provenance. Despite that repeat failure, the journal’s concrete next commitment pivots to `housekeeping-scan` (`docs/journal/2026-03-08.md:111-113`), while making `process-merge --issues` mandatory is demoted to an open question (`docs/journal/2026-03-08.md:115-117`). That is weaker follow-through than the prior cycle promised, especially for a lapse that had already been identified one cycle earlier.

## Recommendations

1. Expand `_build-helper.sh` to track all build-relevant workspace inputs (at minimum `tools/rust/Cargo.toml`) and either teach `tools/pipeline-check` to build its dependent release binaries or fall back to `cargo run` for missing tools.
2. Add an explicit receipt section to cycle worklogs (or another canonical cycle artifact) and require every state-changing step to record the exact short hash used in later audits.
3. Make `process-merge --issues` fail closed when a merge is meant to resolve tracked agent work, and document what to do when provenance such as `model` cannot be recovered during backfill.

## Complacency score

4/5 — Cycle 184 fixed the obvious stale-binary symptom and corrected ADR wording, but it still left the fresh-clone wrapper path broken, carried an unverifiable receipt into the next review, and repeated a session-linking lapse that had already been called out in cycle 183.

## Priority items

1. Fix `tools/pipeline-check` so `bash tools/pipeline-check` works from a fresh clone, not just after manual Rust builds.
2. Close the receipt-verification gap by recording canonical commit hashes in the cycle artifact itself.
3. Remove the recurring `process-merge --issues` footgun instead of treating it as a recurring operator reminder.
