redesign(phase-3): cycle 160 Track 2 — C10 (L2.5) design-scope amendment (cycle 158 forward priority #2 / cycle 159 priority #1 CLOSED)

Bounded textual change to `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md` row for the SuperStepOutOfOrder failure class. Closes the AGREE-ACT-NOW half of the C10 (L2.5) paired verdict from cycle 155's absorption of PR #2951 cycle 152 adversarial critique.

Background (from cycle 155 absorption ledger, _notes line 65–68): the critique observed real contract drift. Design scope said SuperStepOutOfOrder = "Hard error; abort cycle without state mutation; runner exits non-zero." Implementation (`halt_cycle`) calls `write_runner_state` for ALL halt classes including out-of-order before returning Err. The discrepancy was confirmed real. Cycle 155 verdict was AGREE-ACT-NOW + AGREE-WITH-CARVEOUT (paired): amend the design wording to clarify scope (ACT-NOW); preserve the current implementation behavior because it produces useful postmortem data (CARVEOUT).

This commit lands the ACT-NOW half. The amendment clarifies that "state mutation" in the SuperStepOutOfOrder row refers strictly to super-step state-machine state — `state/super-step.json`, channel state envelopes under `state/channels/`, reconciler cursors under `state/reconciler/`, and role-driver-written artifacts. It does NOT forbid the runner from writing its own local observability state (`state/v2-cycle-runner/last-cycle.json`) recording that an out-of-order halt occurred. The boundary tool owns the contractual surface; the runner never directly mutates `state/super-step.json` anyway.

Surface area: 1 file, +3 -1 LOC. No code changes — `halt_cycle` behavior is unchanged.

This amendment also unblocks Missing Integration Scenario 3 from cycle 155 absorption (super-step-out-of-order live integration test): the test can now be written cleanly to assert state-mutation-absent for super-step state and state-mutation-present for runner-local observability state. That test itself remains a future cycle (cycle 155 named it as AGREE-DEFER pending this amendment).

Cycle 160 forward priorities CLOSED: #1 (C10 design-scope amendment).
