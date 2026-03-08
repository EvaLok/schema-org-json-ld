# Cycle 183 Review

## Findings

1. **PR #719 was merged, but its field-refresh fix was not actually exercised in cycle 183**
   Category: stale-binary-cycle-complete

   The worklog says PR #719 merged and that cycle 182’s `duration-freshness-gap` finding was actioned (`docs/worklog/2026-03-08/021634-hundred-eighty-third-orchestrator-cycle.md:5-7`), and the merged Rust change does add the seven intended auto-refresh targets to `cycle-complete` (`tools/rust/crates/cycle-complete/src/main.rs:90-97`, `tools/rust/crates/cycle-complete/src/main.rs:383-401`). But the final cycle-183 state still leaves five of those target fields at `cycle 182`: `pre_python_clean_cycles`, `publish_gate`, `review_agent.chronic_category_responses`, `schema_status.in_progress`, and `test_count` (`docs/state.json:2067-2121`). That matches the wrapper’s stale-binary behavior: `tools/cycle-complete` only rebuilds when the release binary is missing, not when source changed (`tools/cycle-complete:8-10`). In practice, cycle 183 closed issue #718 even though the acceptance criterion (“these entries should no longer go stale after cycle-complete”) was not met by the cycle that supposedly consumed the fix.

2. **The ADR catch-up includes factual errors and aspirational claims, not just durable decisions**
   Category: adr-factual-drift

   ADR 0006 says TypeScript parity means identical `toArray() / toJsonLd()` output and names the npm package as `@anthropic-ai/schema-org-json-ld` (`doc/adr/0006-typescript-port-with-parity.md:17-25`). Both parts are wrong. The real package name is `@evabee/schema-org-json-ld` (`package.json:1-4`), and serialization is handled centrally by `JsonLdGenerator` in both languages rather than schema instance methods (`php/src/v1/JsonLdGenerator.php:9-20`, `php/src/v1/JsonLdGenerator.php:39-68`, `php/src/v1/TypedSchema.php:7-9`, `ts/src/JsonLdGenerator.ts:7-58`). ADR 0009 has the same problem in a different form: it claims the write-side tools eliminate manual state-editing errors entirely and that “no more stale fields” remain (`doc/adr/0009-write-side-pipeline-tools.md:32-47`), but the live cycle-183 state already disproves that with multiple stale event-driven freshness markers (`docs/state.json:2067-2121`). These are not nitpicks; if the ADRs are supposed to be the authoritative memory of major decisions, they cannot contain basic product-name mistakes and claims the current state refutes.

3. **Cycle 183 still reports a green pipeline that is not reproducible from a fresh master checkout**
   Category: pipeline-fresh-clone-drift

   The worklog’s “Current state” section records `Pipeline status: 5/5, 11/11 invariants` as if that were a stable property of the repository (`docs/worklog/2026-03-08/021634-hundred-eighty-third-orchestrator-cycle.md:35-40`), and the checklist still presents `bash tools/pipeline-check` as the canonical verification step (`COMPLETION_CHECKLIST.md:7-16`). But `tools/pipeline-check` only auto-builds the top-level `pipeline-check` binary (`tools/pipeline-check:8-10`); the binary then looks for sibling release binaries, so on this fresh clone `bash tools/pipeline-check --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` skipped all five phases and returned `Overall: FAIL`. That is the same drift the cycle-182 review called out (`docs/reviews/cycle-182.md:10-14`), and cycle 183’s own review history still classifies it as one of the two deferred findings (`docs/state.json:2844-2855`). Deferring it is one thing; continuing to write “5/5” without making clear that the green result depends on non-repo state is how false confidence creeps in.

4. **Two of the receipt hashes supplied for verification are invalid**
   Category: receipt-verification-gap

   The checklist explicitly treats receipt hashes as audit inputs that should let a reviewer reconstruct what happened (`COMPLETION_CHECKLIST.md:120-143`). But two of the hashes this review was asked to verify, `f5462f1` and `e5e58df`, do not resolve at all: `git show f5462f1 --stat` and `git show e5e58df --stat` both fail with “unknown revision.” The nearby real commits are `1c1ef01` (`state(process-review): cycle 182 review consumed`) and `cd0ede1` (`state(process-merge): PR #722 merged`), and both are ordinary `docs/state.json` receipts. That means the cycle’s audit trail is already lossy at the point where it is supposed to be most precise: the reviewer cannot verify what was claimed without first repairing the receipt list by hand.

## Recommendations

1. Fix the stale-binary problem at the wrapper level, not just per-tool source level: either rebuild when source is newer than the release binary or stop relying on prebuilt release binaries for correctness-critical cycle steps.
2. Treat ADRs like code: correct ADR 0006 and ADR 0009 immediately, then require factual spot-checking against live files before closing future “ADR debt” issues.
3. Stop recording pipeline status as a bare “5/5” unless it is reproducible from the checked-out repo with the documented command. If the result depends on prebuilt artifacts, say so explicitly.
4. Make receipt capture verifiable at close time: the orchestrator should validate every listed hash with `git rev-parse --verify` before publishing it in a worklog, issue, or closing summary.

## Complacency score

4/5 — cycle 183 did real work, but too much of it was accepted at the narrative level instead of the operational level. The clearest example is #718: the code merged, the issue closed, and the worklog claimed success even though the final state shows the new behavior was not actually in effect. The ADR burst has a similar smell: impressive volume, but at least two records contain claims that fail basic cross-checks against the repository.

## Priority items

1. Make wrapper execution trustworthy after source changes so cycle-complete and the rest of the write-side pipeline cannot silently run stale binaries.
2. Repair the ADR set from cycle 183 so it documents real architecture decisions rather than a mixture of true history and aspirational prose.
3. Fix the receipt/public-audit path so reviewers can verify the exact commits claimed without guessing replacement hashes.
