# Field Inventory Completeness Check
# Per audit #87: programmatic comparison of state.json fields vs field_inventory entries.
#
# Usage (CI or local):
#   jq -r -f tools/check-field-inventory.jq docs/state.json
#
# Orchestrator manual check (allowed in sandbox):
#   jq -r 'keys[]' docs/state.json
#   jq -r '.schema_status | keys[]' docs/state.json
#   jq -r '.field_inventory.fields | keys[]' docs/state.json
#   Then compare the three lists against each other (see STARTUP_CHECKLIST step 5.11).
#
# Excluded from tracking (append-only or static):
#   agent_sessions, schema_status.implemented, schema_status.quality_fixes,
#   schema_status.enums_implemented, schema_status.enum_namespace,
#   schema_status.directory_layout, release, field_inventory,
#   constructor_refactoring (terminal status), typescript_plan.phases (historical),
#   typescript_plan.eva_decisions, typescript_plan.preparatory_artifacts,
#   typescript_plan.audit_enhancements, eva_input_issues.closed_prior_cycles

(.field_inventory.fields | keys) as $inventoried |

# Mutable top-level fields (excluding append-only and static)
([keys[] | select(
  . != "agent_sessions" and
  . != "release" and
  . != "field_inventory" and
  . != "constructor_refactoring"
)] |
map(select(
  ($inventoried | map(. == . or startswith(. + ".")) | any | not) and
  (. as $k | $inventoried | map(startswith($k + ".") or . == $k) | any | not)
))) as $top_gaps |

# Mutable schema_status sub-fields
([.schema_status | keys[] | select(
  . != "implemented" and
  . != "quality_fixes" and
  . != "enums_implemented" and
  . != "enum_namespace" and
  . != "directory_layout"
) | "schema_status.\(.)"] |
map(select(
  . as $path | $inventoried | map(. == $path) | any | not
))) as $schema_gaps |

# Mutable typescript_plan sub-fields
([.typescript_plan | keys[] | select(
  . != "eva_decisions" and
  . != "preparatory_artifacts" and
  . != "audit_enhancements" and
  . != "phases" and
  . != "plan_version" and
  . != "approved_at" and
  . != "issue" and
  . != "qc_coordination_issue" and
  . != "qc_validation_strategy"
) | "typescript_plan.\(.)"] |
map(select(
  . as $path | $inventoried | map(. == $path) | any | not
))) as $ts_gaps |

# Mutable eva_input_issues sub-fields
(["eva_input_issues.closed_this_cycle"] |
map(select(
  . as $path | $inventoried | map(. == $path) | any | not
))) as $eva_gaps |

($top_gaps + $schema_gaps + $ts_gaps + $eva_gaps) | sort |
if length == 0 then
  "PASS: All mutable fields have field_inventory entries (\($inventoried | length) tracked)"
else
  "GAPS FOUND: \(length) mutable field(s) without inventory entries:\n" +
  (map("  - " + .) | join("\n")) +
  "\n\nCurrently inventoried: \($inventoried | length) fields"
end
