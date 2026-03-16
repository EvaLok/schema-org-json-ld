# Pipeline Stabilization Plan (ADR 0011)

> **Incremental implementation.** This plan is designed to be executed one phase at a time.
> Each phase requires an `input-from-eva` issue before work begins. Do NOT start work on
> any phase that does not have a corresponding open `input-from-eva` issue — Eva controls
> the sequencing and may adjust phase scope or priority between phases.
>
> **Status tracking:**
> - Phase 0: COMPLETE (committed `b38e0a9`, 2026-03-16)
> - Phase 1: ISSUED ([#1350](https://github.com/EvaLok/schema-org-json-ld/issues/1350))
> - Phases 2–5: NOT YET ISSUED — awaiting Eva

---

## Context

After 272 cycles, the orchestrator pipeline still cannot complete cycles without the review agent finding process violations. Chronic issues persist for 10–27 cycles before resolution, and "fixes" frequently introduce new edge cases. The root causes fall into three categories: pipeline design flaws (gates that don't gate), tool bugs (untested code paths), and behavioral variance (orchestrator deviating from its own checklist).

The goal is **convergence to a stable, reproducible pipeline** — a wizard-like hand-off system where each step produces consistent results every time. The system itself (not the library) is the product. Stability is defined as **50 consecutive clean cycles** where "clean" means: pipeline-check all-pass AND no tool/infrastructure modifications dispatched or merged.

Cycles continue running during implementation. The review agent switches to **observation mode** (findings logged, no action items). The system self-implements tool and checklist changes; Eva handles workflow directory changes.

**ADR**: `doc/adr/0011-pipeline-stabilization-program.md`

---

## Phase 0: Stabilization Mode Bootstrap (COMPLETE)

**Goal**: Establish the stabilization operating mode so the system knows it's in a stability-focused phase.

**Scope**: State schema, orchestrator prompt, completion checklist.

### Changes

1. **Add `project_mode` to state.json** (system can do)
   - New top-level field: `project_mode: { mode: "stabilization", started_cycle: 273, clean_cycle_counter: 0, clean_cycle_criteria: { pipeline_all_pass: true, zero_tool_dispatches: true }, consecutive_clean_cycles: [] }`
   - Reuse the `pre_python_clean_cycles` pattern but generalized

2. **Add stabilization directives to orchestrator-prompt.md** (Eva merges)
   - New non-negotiable directive: "When `project_mode.mode` is `stabilization`, do not dispatch tool/infrastructure PRs. Only dispatch schema implementation work or review agent sessions. Any cycle where a tool PR is dispatched or merged resets `clean_cycle_counter` to 0."
   - Modify review dispatch section: "When `project_mode.mode` is `stabilization`, dispatch review agent in observation mode — findings are logged in the review file but NOT treated as action items. No dispatches are triggered by review findings."

3. **Add stability counter update to COMPLETION_CHECKLIST** (system can do)
   - New step between C5.5 and C6: "If `project_mode.mode` is `stabilization`, evaluate clean cycle criteria. If pipeline-check passed AND no tool PRs dispatched/merged this cycle, increment counter. Otherwise reset to 0. Record in state.json."

4. **Modify review agent dispatch in COMPLETION_CHECKLIST** (system can do)
   - Step C6 review dispatch body: add conditional — if stabilization mode, prepend "OBSERVATION MODE: Log findings only. Do not classify as action items. Do not recommend dispatches."

### Files
- `docs/state.json` — add project_mode section
- `.github/workflows/orchestrator-prompt.md` — add stabilization directives (**Eva**)
- `COMPLETION_CHECKLIST.md` — add stability counter step, modify C6

### Exit criteria
- `project_mode.mode` reads "stabilization" from state.json
- Orchestrator acknowledges stabilization mode in cycle startup
- Review agent dispatches include observation mode flag

---

## Phase 1: Gate Enforcement

**Goal**: Make the pipeline gates actually block. Currently all gates are advisory — the orchestrator can (and does) proceed past failures.

**Scope**: Rust tool changes only.

### Changes

1. **`record-dispatch`: Refuse after failed pipeline-check**
   - Before recording a dispatch, run `pipeline-check --cycle N` (or read last cached result)
   - If pipeline-check returned any FAIL status, exit with error: "Cannot dispatch: pipeline-check failed. Fix failures before dispatching."
   - This makes C5.5 → C6 a real gate
   - File: `tools/rust/crates/record-dispatch/src/main.rs`

2. **`state-invariants`: Reject string-valued verification_cycle**
   - Change `check_chronic_intermediate_state` from WARN to FAIL
   - String values like `"270-tool-hardened, pending"` must be resolved to numeric before the cycle can pass
   - File: `tools/rust/crates/state-invariants/src/main.rs`

3. **`process-review`: Remove bypass flag, enforce disposition enum**
   - Remove `--skip-disposition-check` flag entirely
   - Disposition sum MUST equal finding_count — hard error, not warning
   - Validate category values against a fixed list (from chronic_categories keys + known categories)
   - File: `tools/rust/crates/process-review/src/main.rs`

4. **`pipeline-check`: Fail on missing mandatory steps (not just warn)**
   - If any of the 22 mandatory step IDs are missing from posted comments, return FAIL not WARN
   - This makes step-posting a hard requirement, not aspirational
   - File: `tools/rust/crates/pipeline-check/src/main.rs`

### Exit criteria
- `record-dispatch` refuses to run after a failed pipeline-check (test with intentional failure)
- `state-invariants` exits 1 on string-valued verification_cycle
- `process-review` exits 1 on disposition mismatch (no bypass)
- `pipeline-check` exits 1 on missing mandatory steps

---

## Phase 2: Tool Bug Fixes

**Goal**: Fix known correctness gaps in existing tools so they produce reliable results.

**Scope**: Rust tool changes + test additions.

### Changes

1. **`verify-review-events`: Add code-PR test coverage**
   - Current tests only cover docs/tool PRs. Add test cases for actual code PRs that require APPROVED review events, filtering self-reviews and post-merge reviews
   - Test the terminal status filtering (closed_without_merge, failed should not require merge evidence)
   - File: `tools/rust/crates/verify-review-events/src/main.rs`

2. **`cycle-receipts`: Fix dedup test realism**
   - Current test feeds 3 labels for 1 SHA but the parser can only emit 2. Replace with realistic test vectors from actual cycle data
   - Fix receipt rendering that duplicates SHAs across labels (12 rows for 8 receipts)
   - File: `tools/rust/crates/cycle-receipts/src/main.rs`

3. **`cycle-complete --reconcile`: Enforce merged_at on status change**
   - When reconciling a session to status "merged", require and set `merged_at` timestamp
   - Downstream tools (`verify-review-events`) need this field for cycle windowing
   - File: `tools/rust/crates/cycle-complete/src/main.rs`

4. **`write-entry`: Validate scope notes against reality**
   - The "structurally excluded" canned note should be generated by checking whether the current cycle actually has excluded commits, not hardcoded
   - Derive from `cycle-receipts` output or git log comparison
   - File: `tools/rust/crates/write-entry/src/main.rs`

### Exit criteria
- All existing tests pass + new tests for each fix
- `verify-review-events` has at least one code-PR test case
- `cycle-receipts` produces correct row counts matching unique SHAs
- `cycle-complete --reconcile` sets merged_at on every status:"merged" transition

---

## Phase 3: Pipeline Contract Specification

**Goal**: Formalize the wizard — define explicit input/output/success contracts for every pipeline step so the hand-offs are documented and testable.

**Scope**: Documentation + checklist refinement. No tool code changes.

### Changes

1. **Create `docs/pipeline-contract.md`**
   - For each step (0 through C8), define:
     - **Inputs**: What state/artifacts this step reads
     - **Outputs**: What state/artifacts this step produces
     - **Success**: What must be true for this step to pass
     - **Failure**: What happens if this step fails (halt? skip? warn?)
     - **Hand-off**: What the next step expects to find
   - This becomes the authoritative reference for pipeline behavior

2. **Eliminate conditional/optional steps or make them always-run**
   - Steps 0.6, 1.1, 1.5, 2.5, 5.5, 5.6, 5.8, 5.9, 5.10, 5.11, 5.12, 5.13 are conditional
   - For each: either make it mandatory (always runs, produces a pass/skip result) or remove it from the pipeline
   - Conditional steps are the main source of per-cycle behavioral variance

3. **Standardize step output format**
   - Every step comment should include a machine-readable status line: `Status: PASS | SKIP | FAIL`
   - `post-step` tool already validates step IDs; extend it to require a status field
   - This enables automated pipeline analysis without parsing prose

4. **Refine STARTUP_CHECKLIST.md and COMPLETION_CHECKLIST.md**
   - Align with pipeline-contract.md
   - Remove ambiguous language ("consider", "if appropriate", "when relevant")
   - Replace with deterministic conditions ("if X then Y, else skip with reason Z")

### Exit criteria
- `docs/pipeline-contract.md` exists with all steps documented
- Every conditional step has an explicit trigger condition and skip reason
- post-step comments include machine-readable status
- Checklists use deterministic language only

---

## Phase 4: Behavioral Variance Elimination

**Goal**: Remove orchestrator discretion at the specific points where variance has been observed.

**Scope**: Tool changes + checklist changes + prompt additions (some Eva-merge).

### Changes

1. **Disposition auto-generation**
   - `process-review` should read the actual review file and extract dispositions from structured content, rather than accepting manual CLI args
   - The review agent's output format should include a machine-readable disposition section that `process-review` parses
   - Eliminates the mismatch between what the review file says and what the orchestrator reports
   - File: `tools/rust/crates/process-review/src/main.rs`

2. **Review spec immutability during stabilization**
   - The orchestrator currently edits the review spec to suppress findings it doesn't want to see
   - Add a directive: review spec changes require Eva approval during stabilization mode (create `question-for-eva` issue)
   - File: `.github/workflows/orchestrator-prompt.md` (**Eva**)

3. **Canned template elimination in write-entry**
   - All dynamic content in worklog/journal entries must be derived from tool output or git history
   - No hardcoded explanatory notes; if a note is needed, derive it from the data
   - File: `tools/rust/crates/write-entry/src/main.rs`

4. **Chronic category lifecycle automation**
   - Define a state machine for chronic categories: `open → tool_hardened → runtime_verified → closed`
   - Transitions must be driven by tool output (state-invariants for tool_hardened, verify-review-events for runtime_verified), not orchestrator judgment
   - File: `tools/rust/crates/state-invariants/src/main.rs`

### Exit criteria
- `process-review` can parse dispositions from review file (not just CLI args)
- Review spec modifications create a question-for-eva issue during stabilization
- write-entry produces no hardcoded explanatory strings
- Chronic category transitions are tool-driven with no manual override

---

## Phase 5: 50-Cycle Burn-In

**Goal**: Validate stability by running 50 consecutive clean cycles with no tool modifications.

**Scope**: Operational — the system runs itself. No code changes.

### Rules

1. **No tool/infrastructure PRs** dispatched or merged
2. **Review agent in observation mode** — findings logged, not actioned
3. **Pipeline-check must pass** every cycle (now enforced by Phase 1 gates)
4. **Counter resets to 0** on any: tool PR merged, pipeline-check failure, manual state.json edit, gate override
5. **Schema implementation work** continues normally (the library workload)
6. **Counter tracked** in `project_mode.clean_cycle_counter` in state.json

### Observation protocol
- Review agent findings accumulated across 50 cycles
- At burn-in completion, triage all accumulated findings:
  - Patterns that appeared 3+ times → genuine issues to fix post-burn-in
  - One-off findings → likely noise, document and close
  - Findings that disappeared → self-resolving, no action

### Success condition
- `project_mode.clean_cycle_counter >= 50`
- At that point, `project_mode.mode` transitions to `active-development` or `maintenance`
- Review agent returns to normal mode
- Accumulated findings triaged and prioritized

### Failure protocol
- If counter cannot reach 50 after 100 total cycles (50% failure rate), reassess
- Identify the specific steps/tools causing resets
- Those become targeted fix candidates (return to Phase 1 or 2 scope)

---

## Implementation Order

Phases are designed to be independent, but the recommended order is:

```
Phase 0 (bootstrap) --> Phase 1 (gates) --> Phase 2 (bugs) --> Phase 3 (contracts) --> Phase 4 (behavioral) --> Phase 5 (burn-in)
```

- **Phase 0 first**: Sets up the mode flag that other phases reference
- **Phases 1-4 in any order**: Truly independent, can be parallelized
- **Phase 5 last**: Measures the result of all other phases, but can also run as a baseline before any changes

## Who implements what

| Change type | Implementer |
|---|---|
| Rust tool code | System dispatches to Copilot |
| STARTUP/COMPLETION_CHECKLIST.md | System direct-pushes |
| docs/pipeline-contract.md | System direct-pushes |
| docs/state.json schema additions | System via tools |
| .github/workflows/orchestrator-prompt.md | System prepares PR, Eva reviews and merges |
| ADR 0011 | Committed by Eva (2026-03-16) |
