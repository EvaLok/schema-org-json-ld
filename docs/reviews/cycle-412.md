# Cycle 412 Review

## 1. [worklog-accuracy] Cycle state never received the promised post-dispatch refresh

**File**: docs/worklog/2026-03-30/104359-cycle-412-two-merges-thin-test-batch-3-dispatch-metrics-refresh.md:31
**Evidence**: The published worklog still reports `In-flight agent sessions: 1` and only lists the pre-dispatch next step for `#2024`. But the final close-out commit `6b7d23a5` (`state(record-dispatch): #2026 dispatched [cycle 412]`) adds the review issue to `docs/state.json`, changes `in_flight_sessions` from `1` to `2`, and rewrites `last_cycle.summary` to include both `#2024` and `#2026`. This is exactly the stale mixed-timeline problem that the repo already claims to have structurally fixed in `docs/state.json:6862-6863`, where the chronic `worklog-accuracy` response says in-flight sessions, publish gate, and next steps now use post-dispatch addendum lines.
**Recommendation**: Re-enable the post-dispatch addendum path for all worklog state fields before claiming this chronic category is structurally fixed. If the worklog is intentionally a pre-dispatch snapshot, label it that way and add a mandatory post-dispatch section instead of leaving stale counters in the final artifact.

## 2. [process-adherence] Blocking validation and pipeline gates were treated as advisory

**File**: COMPLETION_CHECKLIST.md:98
**Evidence**: The checklist says C4.1 is a **blocking gate** and that the cycle must not proceed to C5 until validation passes (`COMPLETION_CHECKLIST.md:98-111`), and C5.5 says not to dispatch the review agent or close the cycle when the pipeline still fails (`COMPLETION_CHECKLIST.md:172-186`). Cycle 412 did the opposite. On issue `#2023`, the Step C4.1 comment (`https://github.com/EvaLok/schema-org-json-ld/issues/2023#issuecomment-4154031981`) records `Worklog validation: FAIL`. The Step C4.5 comment (`https://github.com/EvaLok/schema-org-json-ld/issues/2023#issuecomment-4154038648`) explicitly says `Override: proceeding with docs commit.` The Step C5.5 comment (`https://github.com/EvaLok/schema-org-json-ld/issues/2023#issuecomment-4154041004`) still records `Pipeline FAIL`, yet Step C6 immediately dispatches the review anyway (`https://github.com/EvaLok/schema-org-json-ld/issues/2023#issuecomment-4154061750`). The current `bash tools/pipeline-check` output still exits non-zero because `doc-validation` fails on the published worklog's pipeline-status mismatch.
**Recommendation**: Make `cycle-runner close-out` fail hard on C4.1/C5.5 violations, or add an explicit waiver mechanism with a committed artifact, state.json marker, and worklog disclosure. An informal “non-actionable override” comment is not equivalent to following the blocking checklist.

## 3. [journal-quality] The journal marks both prior commitments as followed after only proving one

**File**: docs/journal/2026-03-30.md:148
**Evidence**: Cycle 411 ended with two concrete commitments: review PR `#2019`, and investigate the worklog-immutability false positive in resume scenarios with the observable `fix dispatched or design decision documented` (`docs/journal/2026-03-30.md:129-132`). In the cycle 412 follow-through section, both commitments are quoted (`docs/journal/2026-03-30.md:150-151`), but the section then declares `**Followed.**` and only substantiates the first one by describing the merge of PR `#2020` (`docs/journal/2026-03-30.md:153`). There is no corresponding fix dispatch, design decision, or explicit deferral for the second commitment in that follow-through block, so the journal grades the pair as completed without meeting the second observable.
**Recommendation**: Grade commitments one by one and require a disposition for each quoted observable: met, deferred with reason, or dropped with reason. Do not use a single blanket `Followed` label when one of the commitments is still unresolved.

## 4. [infrastructure-consistency] Review dispatch accounting fell back to manual state surgery despite a dedicated tool

**File**: COMPLETION_CHECKLIST.md:39
**Evidence**: The completion checklist says `Do NOT manually edit docs/state.json` and prescribes `bash tools/record-dispatch --issue N --title "Cycle N review" --model gpt-5.4` for review dispatches (`COMPLETION_CHECKLIST.md:39-60`). Cycle 412 admits it did not follow that contract. The Step C6 comment on issue `#2023` says: `record-dispatch auto-detection had false positive on review body containing 'cycle 412 finding' text — recorded manually in state.json` (`https://github.com/EvaLok/schema-org-json-ld/issues/2023#issuecomment-4154061750`). The final commit `6b7d23a5` then modifies only `docs/state.json`, adding issue `#2026`, bumping `in_flight_sessions`, and rewriting `last_cycle.summary`. That means the cycle used the outward form of a `state(record-dispatch)` receipt while acknowledging a manual bypass of the tool that is supposed to own those fields.
**Recommendation**: Fix `record-dispatch` so review bodies cannot trip false positives, and require manual fallbacks to be explicitly labeled as such in the commit message and state. A manual edit that masquerades as a normal tool receipt hides tool failure instead of surfacing it.

## Complacency score

**4/5** — The merged PRs themselves look clean, and the receipt table is reproducible, so this was not a zero-discipline cycle. But the process quality is sliding in a familiar direction: a third consecutive gate bypass was normalized, the published worklog regressed to stale pre-dispatch state despite a claimed structural fix, the journal overstated commitment follow-through, and a dedicated write-side tool was bypassed with a manual state edit. That is more than isolated friction; it is a pattern of treating controls as optional once they become inconvenient.
