# Cycle 169 Review

I verified the listed receipt hashes with `git show <hash> --stat`. The receipts are real and the touched files match the claimed actions, but that verification also shows several of this cycle's "fixed" problems were repaired in `docs/state.json` without aligning the underlying tools or generated operator guidance.

## Findings

1. **The `dispatch_to_pr_rate` contradiction is still live in the code, so the next merge or dispatch can re-break the invariant immediately.**  
   Category: rate-formula-mismatch  
   `state-invariants` now requires `dispatch_to_pr_rate == produced_pr/resolved` (`tools/rust/crates/state-invariants/src/main.rs:422-499`), and the current state was manually repaired to `91/95` (`docs/state.json:631-643`, receipt `38efb67`). But both write-side tools that own this field still compute `produced_pr/total_dispatches`: `process-merge` does `format!("{}/{}", next_produced_pr, total_dispatches)` (`tools/rust/crates/process-merge/src/main.rs:144-146`) and `record-dispatch` does the same via `format_dispatch_to_pr_rate(produced_pr, next_total_dispatches)` (`tools/rust/crates/record-dispatch/src/main.rs:89-93,132-134`). That means the invariant currently passes only because the state was hand-corrected after the fact; the toolchain is still self-contradictory.

2. **The category-extraction fix is incomplete because the automated review prompt still does not require `Category:` annotations, while `process-review` still falls back to slugifying prose.**  
   Category: review-category-contract  
   The issue for this review explicitly required `Category: category-name`, but the standing `cycle-complete` review-agent template still asks only for findings, recommendations, score, and priority items (`tools/rust/crates/cycle-complete/src/main.rs:374-382`). Meanwhile `process-review` continues to extract categories from headings and bold finding titles when no explicit annotation is present (`tools/rust/crates/process-review/src/main.rs:282-317`), which is exactly how cycle 168 ended up with junk long-form categories before a manual cleanup (`38f7f85`, then `d037a81`). Truncating bad slugs helps cosmetically, but the actual contract between review generation and review ingestion is still not encoded in the default tooling.

3. **`cycle-complete` still does not update `last_cycle.number`, yet the checklist tells operators to rely on it for exactly that.**  
   Category: cycle-complete-state-gap  
   The current checklist says `cycle-complete` updates "`last_cycle` fields" and must run before `record-dispatch` "so `last_cycle.number` is updated before `record-dispatch` reads it" (`COMPLETION_CHECKLIST.md:32-39`). But the actual `cycle-complete` state patch writes only `/last_cycle/issue`, `/last_cycle/timestamp`, `/last_cycle/summary`, and `/last_eva_comment_check` (`tools/rust/crates/cycle-complete/src/main.rs:222-249`); it never writes `/last_cycle/number`. `record-dispatch` does in fact read `/last_cycle/number` (`tools/rust/crates/record-dispatch/src/main.rs:76`). The journal and worklog are honest that this gap exists (`docs/journal/2026-03-07.md:15-21,45-54`, `docs/worklog/2026-03-07/034100-hundred-sixty-ninth-orchestrator-cycle.md:42-47`), but the standing operator checklist still overclaims the tool's behavior.

4. **The release-wrapper pattern remains brittle: merged tool changes are not picked up unless someone manually rebuilds, and the pipeline wrapper can degrade into an all-skip run.**  
   Category: stale-release-binaries  
   The shell wrappers only rebuild when the specific release binary is missing, not when the source changed (`tools/process-merge:6-11`, `tools/cycle-complete:6-11`, `tools/pipeline-check:6-11`). That matches the cycle journal's report that `process-merge` remained stale after PR #627 until a manual `cargo build --release -p process-merge` (`docs/journal/2026-03-07.md:45-50`). I also reproduced the broader failure mode locally: before building the full release workspace, `bash tools/pipeline-check --cycle 169` reported `FAIL` because every subordinate tool binary except `pipeline-check` itself was missing and therefore skipped. This is not just an operator gotcha; it is a repeatable infrastructure hazard in the wrapper strategy.

## Recommendations

1. Pick one definition of `dispatch_to_pr_rate` and enforce it everywhere in the same PR: invariant checker, `process-merge`, `record-dispatch`, tests, and any prose that explains the metric.
2. Update `cycle-complete`'s generated review prompt so `Category: <kebab-case>` is mandatory for every finding, and treat missing category annotations as an ingestion error instead of silently deriving categories from arbitrary prose.
3. Either teach `cycle-complete` to write `last_cycle.number` or stop claiming that it does in `COMPLETION_CHECKLIST.md`; the tool behavior and operator instructions need to match.
4. Replace the "build only if missing" wrapper behavior with a predictable rebuild strategy for tool workflows that depend on freshly merged Rust code, or document and automate the required workspace rebuild step before end-of-cycle operations.

## Complacency score

4/5 — there was real cleanup this cycle, but too many "fixes" still depended on manual state surgery and operator memory while the tools and checklist continued to disagree with each other.

## Priority items

1. Resolve the `dispatch_to_pr_rate` formula mismatch in code instead of repairing `docs/state.json` by hand again.
2. Encode review categories into the default `cycle-complete` prompt so `process-review` receives structured input every cycle.
3. Eliminate the stale-release-binary trap before the next cycle that merges Rust tool changes and immediately needs to use them.
