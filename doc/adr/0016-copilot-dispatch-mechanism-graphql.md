# ADR 0016: Copilot Dispatch Mechanism — GraphQL Replaces REST `agent_assignment`

## Status

Accepted (2026-05-08). Applies to all Copilot dispatches from this repository (production and redesign mode). Implemented in `tools/rust/crates/record-dispatch/src/lib.rs` (new `assign_copilot_agent` helper), `tools/rust/crates/dispatch-task/src/main.rs` (rewrite of issue payload + assignment step + `--skip-pipeline-gate` flag), `tools/rust/crates/dispatch-review/src/main.rs` (parallel rewrite), and `.github/workflows/orchestrator-redesign-prompt.xml` (new `<copilot-dispatch-method>` section). Verified end-to-end via test issues [#2879](https://github.com/EvaLok/schema-org-json-ld/issues/2879) and [#2881](https://github.com/EvaLok/schema-org-json-ld/issues/2881).

## Context

From cycle 16 onwards (2026-04-28+), Copilot dispatches stopped wakening the coding agent. Issues were created with `agent-task` + `research-only` / `feedback-only` labels, but Copilot never connected. As of 2026-05-08 there were six issues stuck waiting:

- [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) oh-my-codex deeper read (29 cycles waiting)
- [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842) PAI deeper read (21 cycles)
- [#2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847) oh-my-claudecode survey (17 cycles)
- [#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851) symphony survey (15 cycles)
- [#2869](https://github.com/EvaLok/schema-org-json-ld/issues/2869) Phase 2 candidate critique (cycle 93)
- [#2870](https://github.com/EvaLok/schema-org-json-ld/issues/2870) cycle 91-92 sharpening critique (cycle 93)

The closed-loop cold-reader iteration that ADR 0015 was meant to break depended on the orchestrator successfully dispatching new research. With dispatch silently broken, ADR 0015 could only deliver substantive-focal change in the form of orchestrator-internal work (cross-system synthesis, implications mining, retrospectives). The deeper-read research front never advanced.

Three layers of breakage compound to produce the symptom:

1. **GitHub API** — REST `/issues` POST with `"assignees": ["copilot-swe-agent[bot]"]` no longer reliably triggers Copilot. The REST `/assignees` endpoint rejects bot logins entirely; the legacy `agent_assignment` REST field appears to have stopped having effect. The only verified working path to assign the Copilot coding agent is the GraphQL `replaceActorsForAssignable` mutation.

2. **`tools/dispatch-task` and `tools/dispatch-review` Rust binaries** codified the broken REST template literally. Their unit tests asserted on the broken payload shape (`assignees == ["copilot-swe-agent[bot]"]`, `agent_assignment.target_repo == MAIN_REPO`, etc.), locking the bug in.

3. **Redesign-mode prompt** (`orchestrator-redesign-prompt.xml`, in use since 2026-04-27) had a `<copilot-dispatches>` section authorizing the three dispatch types but no `<dispatch-method>` block specifying how to actually execute one. The orchestrator improvised with raw `gh issue create --label agent-task --assignee EvaLok`, which doesn't even attempt bot assignment. The production prompt did have a `<dispatch-method>` block, but it pointed at the broken layer-2 template.

Verification (2026-05-08):

- Manual GraphQL `replaceActorsForAssignable` mutation against the six stuck issues unblocked them within seconds — Copilot connected ~10s after assignment for each. Within hours, all six produced Copilot working branches.
- An isolated test workflow ([`.github/workflows/copilot-dispatch-test.yml`](../../.github/workflows/copilot-dispatch-test.yml)) running `claude-code-action@v1` with `ORCHESTRATOR_PAT` driving `gh api graphql` reproduced the same success: test issues #2879 and #2881 both showed `assigned` and `connected` events from Copilot within 10 seconds of mutation. Mechanism works identically from CI as from interactive sessions.

## Decision

Replace the REST-based dispatch mechanism with a two-step flow:

1. Create the issue via REST POST with a minimal payload (title, body, labels). Drop `assignees` and `agent_assignment` fields entirely — they were dead weight at best and active footguns at worst.
2. After successful creation, call the GraphQL `replaceActorsForAssignable` mutation using the issue's `node_id` (returned by the REST POST response) to assign actor IDs `BOT_kgDOC9w8XQ` (`copilot-swe-agent`) and `U_kgDOALttcg` (`EvaLok`).

**Implementation:**

- `record_dispatch::assign_copilot_agent(issue_node_id)` — shared helper that runs the GraphQL mutation via `gh api graphql`. Constants `COPILOT_AGENT_ACTOR_ID` and `EVALOK_USER_ACTOR_ID` are exposed publicly with re-discovery queries documented in their doc comments.
- `dispatch-task` and `dispatch-review` rewritten: `IssuePayload`/`ReviewIssuePayload` reduced to `{ title, body, labels }`; `CreatedIssue` extended with `node_id`; both crates call `assign_copilot_agent` after `create_issue` succeeds. Failures in the assignment step log a warning but do not roll back the dispatch — the issue exists, and assignment can be retried manually.
- New `--skip-pipeline-gate` flag on both crates. The pipeline-check gate (`dispatch-task`) and C5.5 review-dispatch gate (`dispatch-review`) are not maintained in redesign mode; the flag bypasses them with an explicit log line. The gates remain enforced by default for production-mode use.
- Redesign prompt updated with a new `<copilot-dispatch-method>` section under `<copilot-dispatches>` instructing the orchestrator to use `tools/dispatch-task --skip-pipeline-gate`. Raw `gh issue create` for Copilot dispatches is explicitly forbidden.

**Scope:** All Copilot dispatches from this repository, production and redesign modes. The production prompt's `<dispatch-method>` block still references the old broken REST template; that is out of scope for this ADR (production mode is dormant during redesign), but should be updated when redesign mode ends.

## Consequences

### Positive

- Copilot dispatches actually wake Copilot. Six stuck dispatches unblocked (#2833, #2842, #2847, #2851, #2869, #2870); all produced working branches within hours.
- The polarity directive from ADR 0015 (research expansion as default substantive focal) becomes operationally effective. Cycle-63+ deeper-read dispatches that were blocked at the dispatch layer can now actually run.
- The dispatch tool surface (`tools/dispatch-task`, `tools/dispatch-review`) is restored as the canonical path. Future orchestrator changes that touch dispatching benefit from the centralized state-recording, receipt-commit, and (when the gates are enforced) pipeline integration.
- Bot/user actor IDs are documented as constants with re-discovery queries — when GitHub rotates or the IDs change, the failure mode is a clear error from the GraphQL mutation, and the recovery path is the documented query.

### Negative

- The actor IDs are hardcoded. If GitHub rotates bot IDs without notice, dispatches will silently fail until the IDs are re-discovered and updated. Mitigation: the `assign_copilot_agent` failure path explicitly references the re-discovery query and ADR 0016, and the integration test workflow can be re-run on demand to detect breakage.
- The pipeline gate is now bypassable with an explicit flag. Future production-mode use must be careful not to leave `--skip-pipeline-gate` enabled by default. Mitigation: the flag prints a warning to stderr each time it is used, and the redesign-mode prompt is the only place it is currently invoked.
- `assign_copilot_agent` failures are logged but do not roll back the dispatch. A failure produces an issue without Copilot assigned, requiring manual intervention. Acceptable trade: silent rollback would mean a dispatch the orchestrator believes failed but the issue actually exists, which is worse.
- `agent_assignment.model` was the previous knob for selecting `gpt-5.4` vs `gpt-5.5` per dispatch. With the field removed, Copilot uses the user/org default model. If per-dispatch model selection becomes important, it will need a separate mechanism (likely a follow-on GraphQL mutation or Copilot configuration API).

### Trade-offs

- Hardcoded actor IDs vs runtime discovery: chose hardcoded for simplicity and to avoid an extra round-trip per dispatch. Discovery query is documented for the rotation case.
- Two REST + GraphQL calls vs single combined mechanism: GraphQL doesn't allow mixing a query and a mutation in one operation, and the REST POST response already includes `node_id`, so two calls is the minimum. Acceptable.
- Adding `--skip-pipeline-gate` vs auto-detecting redesign mode: chose explicit flag. Auto-detection couples dispatch mechanics to mode-state coupling, and gives no audit trail when the gate is bypassed. Explicit flag with stderr log is auditable.

## Alternatives Considered

**B — Inline GraphQL mutation in the redesign prompt only (Shape A from initial diagnosis).** Rejected: leaves `tools/dispatch-task` and `tools/dispatch-review` broken as latent debt for production-mode return. Two paths for the same operation invites drift. The deeper fix is roughly the same effort once test fixture updates are factored in.

**C — Update REST `/assignees` POST endpoint with bot login.** Rejected: GitHub's REST endpoint silently drops bot logins. Verified by inspecting the working pre-bug issue (#2762) — both `Copilot` and `EvaLok` appear in the assignees array, which would only be possible via a GraphQL path or an internal-only REST extension that has since been removed.

**D — Discover actor IDs at runtime via `suggestedActors` query on every dispatch.** Rejected: adds latency and an extra failure surface to every dispatch for a problem (ID rotation) that has not been observed. Hardcoded constants with documented re-discovery is a better trade.

**E — Restore the `agent_assignment` REST field via undocumented API.** Rejected: even when the field was effective, it was a non-standard REST extension and offered no transparency on what it did. GraphQL `replaceActorsForAssignable` is documented, stable, and the same mechanism the GitHub UI uses internally for assignment changes.

## Cross-references

- ADR 0015 — Cycle Composition Polarity for Redesign Mode. ADR 0015's effectiveness was gated on dispatches actually firing; this ADR removes that gate.
- Issue [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) — input-from-eva directive announcing the polarity inversion (cycle 62).
- PR [#2830](https://github.com/EvaLok/schema-org-json-ld/pull/2830) — polarity directive merge (commit `fcbb9023`).
- Test issues [#2879](https://github.com/EvaLok/schema-org-json-ld/issues/2879) and [#2881](https://github.com/EvaLok/schema-org-json-ld/issues/2881) — end-to-end CI verification of the GraphQL mechanism.
- Stuck dispatches: [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833), [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842), [#2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847), [#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851), [#2869](https://github.com/EvaLok/schema-org-json-ld/issues/2869), [#2870](https://github.com/EvaLok/schema-org-json-ld/issues/2870) — all unblocked manually via GraphQL mutation, then producing Copilot branches.
- Working pre-bug reference: PR [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763) (cycle 15 AutoGen research, authored by `app/copilot-swe-agent`) — last successful Copilot dispatch before the regression.
- Production prompt: `.github/workflows/orchestrator-prompt.xml` SECTION 6 `<coding-agent>` — contains the broken REST template (lines 408-428); out of scope for this ADR but should be updated when redesign mode ends.
- Test workflow: `.github/workflows/copilot-dispatch-test.yml` and `.github/workflows/copilot-dispatch-test-prompt.md` — preserved as a smoke test for future auth/scope or actor-ID changes.
