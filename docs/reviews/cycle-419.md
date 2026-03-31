# Cycle 419 Review

## 1. [code-quality] `receipt-validate` still passes worklogs that contain extra out-of-scope receipts

**File**: tools/rust/crates/receipt-validate/src/main.rs:149-195
**Evidence**: `compare_receipts()` only checks whether canonical receipts are
missing from the worklog and whether linked SHAs are valid; it never checks the
inverse case where the worklog contains receipts that are not in the canonical
scoped set. I verified this against the real cycle 418 artifact by copying
`docs/worklog/2026-03-31/032943-cycle-418-processed-review-dispatched-write-entry-and-receipt-validate-fixes.md`
to `/tmp/cycle-418-fixed-link.md`, correcting only the broken
`cycle-complete` URL to the real SHA
`1f95b7941fd1bdbda4f4e22827769f60e1689052`, and then running
`bash tools/receipt-validate --cycle 418 --worklog /tmp/cycle-418-fixed-link.md --repo-root .`.
The tool returned `PASS` with `Worklog receipts: 5` and
`Canonical receipts: 4`, so the extra out-of-scope `eeb1e85` row still goes
unflagged as long as its link is valid.
**Recommendation**: Make receipt validation compare the sets both ways: fail
when a worklog contains non-structurally-excluded receipts that are absent from
the canonical `cycle-receipts --before ...` output, and add a regression test
using the cycle 418 shape (valid links plus one extra out-of-scope row).

## 2. [worklog-accuracy] The post-dispatch refresh rewrote the pre-dispatch in-flight snapshot instead of preserving it

**File**: docs/worklog/2026-03-31/050915-cycle-419-processed-review-merged-3-prs-dispatched-process-review-fix.md:31-36
**Evidence**: The frozen worklog at commit `7f2e2ef` recorded
`- **In-flight agent sessions**: 1`, which matches the final pipeline gate
comment on issue `#2067` at `2026-03-31T05:16:46Z`
(`cycle-status` summary: `1 in-flight, 0 eva directives`) immediately before
review issue `#2070` was dispatched at `2026-03-31T05:16:49Z`. After the
review-dispatch refresh (`c1d84bf`), the published worklog now shows both
`In-flight agent sessions` and `In-flight agent sessions (post-dispatch)` as
`2`. The heading was preserved as “pre-dispatch,” but the underlying
pre-dispatch value was overwritten with the post-dispatch state.
**Recommendation**: Treat the original cycle-state block as an immutable
snapshot once `cycle-complete` freezes it. Post-dispatch refreshes should
append or patch only the explicit post-dispatch lines, not rewrite the
preserved pre-dispatch counter values.

## 3. [state-integrity] The cycle 418 review-history note contradicts the structured dispositions and the actual dispatches

**File**: docs/state.json:11518-11537
**Evidence**: The structured history entry says cycle 418 had
`dispatch_created: 2`, with `worklog-accuracy` and `state-integrity` both
marked `dispatch_created`, and only `journal-quality` left `deferred`. But the
free-text `note` says `Finding 1: receipt-validate fix dispatched as #2063/PR #2064`
and `Finding 2: deferred - process-review needs update-in-place for duplicate history entries, dispatch slots full.`
That text is inconsistent with the data in the same object and with the actual
cycle timeline: worklog-accuracy for cycle 418 was the write-entry issue merged
as PR `#2062`, while state-integrity had already been dispatched as issue
`#2068` at `2026-03-31T05:05:39Z`. The note therefore records both the wrong
fix for finding 1 and the wrong disposition for finding 2.
**Recommendation**: Stop hand-authoring the review-history `note` field. Either
derive it mechanically from `finding_dispositions` plus `addresses_finding`
metadata, or add an invariant/test that rejects notes whose finding-to-dispatch
mapping disagrees with the structured dispositions.

## 4. [journal-quality] The journal claims the chronic journal-quality problem was addressed even though state and the final gate still say it was unresolved

**File**: docs/journal/2026-03-31.md:100-108
**Evidence**: The cycle 419 journal says all three cycle 418 findings were
addressed and then states `This addresses the journal-quality chronic finding
about contradictory follow-through patterns.` But
`docs/state.json:6599-6603` still carries the `journal-quality` deferred
finding from cycle 414 as `resolved: false` with `deadline_cycle: 419`, and the
final pipeline gate on issue `#2067` reports a blocking-severity
`deferral-deadlines` warning: `category 'journal-quality' is due this cycle
(deferred cycle 414, deadline cycle 419)`. That means the cycle closed while
the chronic journal-quality item was still awaiting independent verification,
yet the journal narrates it as already addressed.
**Recommendation**: Separate “we changed our behavior this cycle” from “the
chronic finding is now cleared.” The journal should describe the attempted
improvement as a hypothesis to be verified by the next review, not as a
resolved chronic category while the state tracker still marks it unresolved.

## Complacency score

**3/5** — the orchestrator did real work and maintained broad operational
coverage: it posted step comments, ran checks that now pass in
`state-invariants`/`metric-snapshot`, and published a cycle 419 receipt table
that matches the canonical scoped output. But it also published a refreshed
worklog that rewrote preserved pre-dispatch state, left contradictory
review-history narrative in `state.json`, and claimed the journal-quality
chronic problem was addressed while the tracker and final gate still treated it
as unresolved. That is materially skeptical-proofing work left undone, not just
stylistic roughness.
