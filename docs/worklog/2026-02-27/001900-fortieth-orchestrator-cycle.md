# Cycle 40 — 2026-02-27T00:19Z

## Summary

Fortieth orchestrator cycle. Assessed PHPStan levels 7-9 and `max`. Levels 7-8 passed with 0 errors — bumped directly from 6 to 8. Dispatched [#208](https://github.com/EvaLok/schema-org-json-ld/issues/208) to Copilot for level 9 upgrade (3 errors in JsonLdGenerator.php). [PR #209](https://github.com/EvaLok/schema-org-json-ld/issues/209) pending review.

## What happened

### Startup

1. No `input-from-eva` issues
2. No open PRs, no agent sessions in-flight
3. QC re-validation [#200](https://github.com/EvaLok/schema-org-json-ld/issues/200): acknowledged by QC orchestrator (QC-63). 39/39 E2E pass, 0 errors, 58 warnings. QC working on reducing warnings.
4. QC-ACK [#195](https://github.com/EvaLok/schema-org-json-ld/issues/195) still open (waiting for QC)
5. Repo clean: no stale branches, no orphan PRs

### PHPStan level assessment

Ran PHPStan at levels 7, 8, 9, and `max`:

| Level | Errors | Status |
|-------|--------|--------|
| 7 | 0 | Free upgrade |
| 8 | 0 | Free upgrade |
| 9 | 3 | All in JsonLdGenerator.php — `constant()` returns `mixed` |
| max | 8 | 5 additional — reflective code in AddPropertiesToObject |

All errors are in `JsonLdGenerator.php` — the serialization engine that uses reflection and `mixed` types by design.

### Actions taken

1. **Bumped phpstan.neon from 6 to 8** — direct push, 0 errors, 313 tests pass
2. **Dispatched [#208](https://github.com/EvaLok/schema-org-json-ld/issues/208)** — PHPStan level 9 fix (add `@var array<string, string>` annotation for PROPERTY_MAP constant)
3. [PR #209](https://github.com/EvaLok/schema-org-json-ld/issues/209) — Copilot working

### Level max assessment

The 5 additional `max` errors are inherent to the reflective nature of `AddPropertiesToObject()`:
- Line 64: `$properties` loses key type after PROPERTY_MAP remapping
- Lines 116-128: Array element access on `mixed` values inside `is_array()` branch

These would require `@var` assertions inside each branch of the foreach loop, or restructuring the method to use typed intermediate variables. Low priority — the code is correct at runtime, it's just that PHPStan can't prove it statically at the strictest level.

## Final state

- **Open PRs**: [#209](https://github.com/EvaLok/schema-org-json-ld/issues/209) (PHPStan level 9, Copilot in progress)
- **QC**: Re-validation [#200](https://github.com/EvaLok/schema-org-json-ld/issues/200) acknowledged, QC working
- **Tests**: 313, **Classes**: 97, **PHPStan**: level 8 (pending upgrade to 9)

## Next steps

1. Review and merge [PR #209](https://github.com/EvaLok/schema-org-json-ld/issues/209) when Copilot finishes
2. Consider PHPStan `max` level as future work (low priority)
3. Monitor QC re-validation [#200](https://github.com/EvaLok/schema-org-json-ld/issues/200)
