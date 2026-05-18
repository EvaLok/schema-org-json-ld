# Schema-promotion discipline — `schema-promotion-requires-reader-co-edit-via-named-field`

**Status:** discipline note at NOVEL@1 (one observed instance, cycle 172).
HARDENING criterion: RECURRENCE-AT-2 from forward-watch cycles 173-180
would motivate codification into a workspace lint or test.
**Originated:** cycle 172 _notes.
**Authored:** cycle 175 (anticipatory codification; cycle 175 _notes
Track 2).

## 1. The pattern

When a `#[serde(flatten)] extra: BTreeMap<String, Value>` field on a
serde-deserialized struct is promoted to a typed named field, **every
reader that previously accessed the value via `.extra.get("...")` must be
co-edited** to use the typed field. Serde-flatten's deserialization
contract is that named-field deserialization takes priority over the
catch-all `extra` map. After promotion, `.extra.get("the_promoted_key")`
returns `None`; readers that don't migrate are silently broken.

## 2. The cycle 172 instance

The pattern was named from a concrete workspace-test regression:

- `state-schema::ReviewHistoryEntry` carried `review_issue` via the
  `extra` map (cycle 167-era schema).
- Cycle 172 promoted `review_issue: Option<u64>` to a typed named field
  to support write-entry's history-fallback resolution.
- The promotion broke `write-entry::review_history_entry_matches_target`,
  which had been reading via `entry.extra.get("review_issue")`.
- The workspace-test `worklog_auto_review_summary_reports_all_same_
  dispositions` regressed.
- Recovery: co-edit `review_history_entry_matches_target` to use
  `entry.review_issue` (the typed field).

The cycle 172 fix landed in commit `208d8d69` with the co-edit included
alongside the field promotion. Master Rust CI went green on that commit
for the first time in 7 days (closing the cycle 171 NOVEL@1 instance of
`master-CI-red-not-noticed-across-multiple-cycles`).

## 3. When the rule applies

Trigger conditions (all must hold):

1. A serde-deserialized struct includes `#[serde(flatten)] extra:
   BTreeMap<String, Value>` (or equivalent).
2. A new typed named field is added for a key that was previously
   accessible only via the `extra` map.
3. Existing code reads the field via `.extra.get("the_key")` (or any
   equivalent of "ask the extra map for that name").

When all three hold, the promotion is a breaking change for the readers
at point 3.

## 4. What readers must check

Pre-promotion sweep (mandatory):

1. **Identify the migrating key.** The exact string literal that
   `.extra.get(...)` calls would use.
2. **Grep the workspace.** `grep -rn '\.extra\.get("the_key")' tools/
   --include='*.rs'` produces the list of readers.
3. **Co-edit each reader.** Replace `.extra.get("the_key")` with the
   typed-field access (e.g., `&entry.the_key`). Adjust types as
   appropriate (`Option<T>` semantics may shift if the typed field is
   itself optional vs the extra map's `Option<Value>` always-optional
   shape).
4. **Workspace test.** `cargo test --workspace --release` must pass on
   the same commit that includes both the promotion and the reader
   migrations. Splitting promotion-only and reader-migration into
   separate commits is allowed only if the promotion commit is
   immediately followed by the reader-migration commit AND both land
   together on master (the in-between state should not survive past one
   CI run).

Post-promotion verification:

5. **Search for stragglers.** A workspace-test pass is the primary
   signal, but a deliberate grep after promotion catches readers that
   the test suite doesn't exercise.

## 5. Why this is not yet a lint

The cost-vs-value calculation for tooling this pattern at NOVEL@1 is
unfavorable:

- A lint that scans for `.extra.get(...)` patterns and warns when a
  named field shadows the key would catch the pattern, but at the cost
  of false positives on legitimate `extra`-map readers (i.e., readers
  that intentionally want the extra-map fallback for non-typed keys).
- A test-style check (a workspace test that asserts no surviving
  `.extra.get(...)` for any typed-field key) catches the same surface
  but with a runtime cost on every test run.
- At NOVEL@1 (one observed instance, cycle 172), neither investment is
  justified.

The cycle 172 instance was already caught by the existing workspace test
(`worklog_auto_review_summary_reports_all_same_dispositions`). The
discipline above plus the existing workspace test is sufficient at
NOVEL@1.

## 6. When this note converts to a lint

RECURRENCE-AT-2 (a second observed instance during forward-watch cycles
173-180) would motivate:

1. Promotion of this note from `_notes/schema-promotion-discipline.md`
   to a more durable surface (a discipline section in a top-level doc,
   or a state-schema migration guide).
2. Design-scope work for a workspace check — either a lint, a test, or
   a manual sweep tool — that catches future schema-promotion events
   before they reach master.
3. Update of the cycle 172 forward-watch projection (currently "if
   recurrence, HARDENS to RECURRENCE-AT-2 and motivates a state-schema
   migration discipline").

If forward-watch closes (cycles 173-180 with no recurrence), this note
remains at NOVEL@1 as a captured discipline that future cycles can
reference if a recurrence eventually occurs.

## 7. Related observations

- **The cycle 172 _notes named this pattern** at the time of first
  occurrence; cycle 175 _notes Track 2 produces this discipline note as
  anticipatory codification (capture-without-implementation).
- **`schema-promotion-requires-reader-co-edit-via-named-field` is one
  of multiple cycle-172-era schema-discipline patterns** — see the
  cycle 172 _notes for context on the broader state-schema migration
  context (cross-crate schema asymmetry was the root cause).
- **The workspace test that caught the cycle 172 instance**
  (`worklog_auto_review_summary_reports_all_same_dispositions`) is a
  serendipitous safety net — it was not designed to catch schema
  promotion drift, but it exercises a code path that depends on the
  promoted key's accessibility. Future readers can rely on workspace
  tests as a first-line defense; the discipline above is the
  pre-commit safety net that survives even tests that don't exercise
  the path.

## 8. References

- Cycle 172 _notes — pattern naming and instance:
  [`cycle-172-write-entry-history-fallback-and-master-CI-green.md`](cycle-172-write-entry-history-fallback-and-master-CI-green.md).
- Cycle 172 commit `208d8d69` — the field promotion + reader co-edit
  landing.
- Cycle 175 _notes — anticipatory codification rationale (Track 2):
  [`cycle-175-reconciler-honesty-pass-and-schema-promotion-discipline-note.md`](cycle-175-reconciler-honesty-pass-and-schema-promotion-discipline-note.md).
