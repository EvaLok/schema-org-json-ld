# Voyager (MineDojo/Voyager, research artifact, last commit 2023-07-27)

[← back to Phase 1 index](../../1-research.md)

Cycle 17 orchestrator-direct read. Sources: README on master,
[arxiv 2305.16291](https://arxiv.org/abs/2305.16291) abstract,
`voyager/voyager.py`, `voyager/agents/skill.py`, `voyager/agents/critic.py`,
`voyager/agents/curriculum.py` (first 150 lines), `voyager/prompts/`
listing. The full paper PDF is on `voyager.minedojo.org` (not fetched
this cycle; abstract + code suffice for architecture reading).

**Status as observable evidence.** Voyager is a research-paper artifact
("the first LLM-powered embodied lifelong learning agent in Minecraft").
Last commit on master is 2023-07-27 — codebase stable as a research
reference, not actively maintained. README front-loads the experimental
claim and the three named components; no successor-framework
recommendation (parallel to AutoGen's maintenance-mode signaling, but
the rationale is "research artifact" rather than "framework deprecated").

**Four-agent architecture with explicit named roles.** `voyager/agents/`
contains four classes:
- `ActionAgent` — the "iterative prompting mechanism" in the paper.
  Generates JavaScript code (Mineflayer API) for the next task; consumes
  execution feedback for refinement.
- `CurriculumAgent` — automatic curriculum. Selects what task to attempt
  next from world state, completed-tasks history, and failed-tasks
  history.
- `CriticAgent` — self-verification. Returns structured
  `{success: bool, critique: str}` from environment events vs task
  statement.
- `SkillManager` — persistent skill library. Stores executable code +
  LLM-generated description + vector embeddings.

**Cost tiering across agents.** Different agents use different model
tiers in default configuration:
- ActionAgent, CurriculumAgent (main), CriticAgent: `gpt-4`
- CurriculumAgent QA-cache lookups, SkillManager skill-description
  generation: `gpt-3.5-turbo`

The pattern: more-expensive model for novel reasoning (action-code
generation, curriculum-task selection, critic-verification); cheaper
model for cached/derivative work (caching Q&A about world state,
generating descriptions of just-written code).

**Component-local persistence.** Each agent persists state in its own
subdirectory under `ckpt/`:
- `ckpt/skill/` — skill library (JSON manifest, per-skill `.js` and
  `.txt` files, Chroma vectordb)
- `ckpt/curriculum/` — `completed_tasks.json`, `failed_tasks.json`,
  `qa_cache.json`, vectordb of cached questions
- `ckpt/action/` — action-agent chat log
- `ckpt/event/` — event recorder for environment events

No central state file; resume is opt-in per-agent (`resume=True`).
Parallel to AutoGen's component-local-dictionaries pattern (different
domain, similar shape).

**Sync invariants asserted at init.** SkillManager asserts
`vectordb._collection.count() == len(self.skills)` at construction;
CurriculumAgent asserts the same for the QA-cache vectordb vs
`qa_cache.json`. Error messages name the failure mode and remediation
("Did you set resume=False ... You may need to manually delete the
vectordb directory"). Dual-storage divergence is a fail-fast condition
at boot, not a silent runtime error.

**Skill versioning is append-on-disk, replace-in-vectordb.** When
`add_new_skill` runs on an existing skill name, the vectordb entry is
deleted and re-added with the new version; the new code is written to
`<name>V2.js`, `<name>V3.js`, ... — old code is never deleted from the
filesystem. The active retrieval surface (vectordb) is single-version;
the disk is monotonic-append history.

**Skill retrieval as semantic similarity.** `retrieve_skills(query)`
returns top-k (default 5) skills by similarity to the query embedding,
where embeddings are over LLM-generated skill descriptions, not raw
code. Retrieved skills get composed into action prompts as available
context.

**Iteration mechanism with bounded retries.**
`action_agent_task_max_retries = 4`. On action failure, the critic's
critique + execution error feeds into the next action prompt; the
action agent rewrites code for the same task. Skills are added to the
library only on `success=True`. Failed tasks accumulate in
`failed_tasks.json`; the curriculum agent uses both completed and
failed history when selecting the next task. Failure is a recorded
artifact, not just a transient.

**Mode toggleability for human-in-the-loop.** CurriculumAgent:
`mode="auto"` (LLM-selected tasks) or `mode="manual"` (human-curated).
CriticAgent: `mode="auto"` (LLM-verified) or `mode="manual"` (human
verifies via stdin prompts). Human-in-the-loop is a configurable mode,
not the architectural default; the manual codepaths are explicit
methods (`human_check_task_success`).

**Curriculum warm-up gates context disclosure based on progress.** The
curriculum's `default_warmup` dict gates which world-observation fields
appear in the curriculum prompt based on completed-tasks count.
Example: `"context": 15` means context-elaboration is hidden until ≥15
tasks completed; `"hunger": 15` similarly delays hunger-reasoning.
Newer agents see less; matured agents see more. Capability disclosure
is a function of progress, not fixed configuration.

**No model fine-tuning.** Per the README: "Voyager interacts with GPT-4
via blackbox queries, which bypasses the need for model parameter
fine-tuning." Learning happens through skill-library accumulation and
prompt-context updates, not gradient updates. The architecture decision
is explicit and load-bearing in the paper's framing.

**Two-layer capability composition.** Two layers of code are made
available to the action agent in prompts:
- `voyager/control_primitives/` — hand-written low-level Mineflayer
  primitives (e.g., `mineBlock`, `craftItem`, `placeItem`).
- Skill library — LLM-generated compositions of primitives (and earlier
  skills).

Skills compose primitives; later skills compose earlier skills.
Compositionality is the paper's named learning mechanism.

**Prompts as external files split by sub-task.** `voyager/prompts/`
contains 8 distinct prompt templates: `action_template.txt`,
`action_response_format.txt`, `critic.txt`, `curriculum.txt`,
`curriculum_qa_step1_ask_questions.txt`,
`curriculum_qa_step2_answer_questions.txt`,
`curriculum_task_decomposition.txt`, `skill.txt`. Curriculum's
task-selection is split across three prompt files for sub-decisions.
Code handles variable injection; prompts hold instructions.

**Anchoring caveats on Voyager.** These caveats argue *non-transfer*:
each names a difference between Voyager's substrate and the redesign's
substrate that may discount specific patterns. The reverse — which
patterns DO transfer despite these differences — is not derivable from
this list alone. Transferability requires a positive argument per
pattern, not just absence of a discount-reason in this section. The
asymmetry is worth naming because confirmation-bias-on-aligned-
principles (failure mode #1 in the anchoring discipline section) has a
counterpart failure mode of over-discounting via blanket caveats.
- **Continuous-runtime vs cold-cycle.** Voyager runs as a single
  process holding agent state in memory across many tasks; the
  redesign target runs in 75-minute cycles with cold restarts between.
  Voyager's "lifelong" continuity is a runtime property; the redesign
  must reconstruct equivalent continuity from disk every cycle.
- **Embodied environment with rich observations vs sparse repository
  state.** Voyager's "world" is a Minecraft instance with
  biome/inventory/voxels/entities/health observations every step; the
  redesign's "world" is a git repo + GitHub issues + cron triggers,
  with much sparser per-step observation surface.
- **Concrete execution feedback vs fuzzy outcome feedback.** Voyager
  skills succeed or fail by concrete code execution + critic check on
  environment events; the redesign's outcome feedback is fuzzier
  (next-cycle retrospection, audit critique, schema-output quality).
- **Skill = executable code in a sandbox vs tool = build-time
  artifact.** Voyager's skills are LLM-generated JavaScript run in an
  external Node.js Mineflayer process; the redesign's tools are Rust
  binaries built at repo-time and reviewed by humans. Voyager's skill
  discipline is at runtime; the redesign's tool discipline is at
  construction.
- **Single agent vs multi-orchestrator.** Voyager has one agent in one
  runtime; the redesign system already runs two orchestrators (main +
  audit) on independent crons.
- **Internal curriculum vs externally-supplied curriculum.** Voyager's
  curriculum agent autonomously selects what to learn next; the
  redesign's "curriculum" is provided by Eva, schema-org work, and the
  retrospective F-patterns.
- **Single-LLM-vendor coupling vs multi-vendor.** Voyager hardcodes
  ChatOpenAI / OpenAIEmbeddings (langchain bindings); the redesign uses
  Claude (Anthropic) for orchestration and Copilot (multiple OpenAI
  models) for dispatches. Vendor-coupling assumptions in Voyager don't
  transfer.
- **Research artifact vs production-grade target.** Voyager is
  unmaintained since 2023-07-27; pattern observations should be
  evaluated as "the design choices a research project documented" not
  "the design choices a production-stable framework converged on."
  Some patterns may have been chosen for paper-narrative reasons rather
  than long-run robustness.

**Patterns observed in Voyager** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Four-agent architecture with explicit named roles (action, curriculum,
  critic, skill-library)
- Cost-tiering across agents: cheap model for cached/derivative work,
  expensive model for novel reasoning
- Component-local persistence with `resume=True` opt-in per agent (no
  central state file)
- Sync invariants asserted at init for dual-storage components (vectordb
  count vs JSON manifest count, fail-fast on divergence)
- Skill versioning as append-on-disk + replace-in-vectordb (active
  surface single-version, history monotonic)
- Top-k semantic skill retrieval via vector similarity over generated
  descriptions (not raw code)
- Bounded retries on action failure with critic-critique + execution-error
  fed into next prompt
- Failed-task accumulation in dedicated JSON file alongside
  completed-task accumulation (failure as recorded artifact, not
  transient)
- Human-in-the-loop as configurable mode (`auto`/`manual`) per agent,
  not architectural default
- Curriculum warm-up gating which observation fields are disclosed based
  on completed-tasks count
- Explicit no-fine-tuning architectural commitment, with skill-library
  as the named learning mechanism
- Two-layer capability composition: hand-written primitives + LLM-composed
  skills over primitives
- Compositionality (skills compose primitives; later skills compose earlier
  skills) as the paper's named learning mechanism within the skill-library
  architecture
- Prompts as external files split by sub-task (curriculum decomposed
  across three prompt files for sub-decisions)
- Structured critic output (`{success: bool, critique: str}`) rather
  than free-form review
- LLM-generated skill descriptions as the embedding surface (embeddings
  are over descriptions, not over raw code)
- QA-cache pattern for repeated curriculum lookups (`qa_cache.json`
  plus vectordb of cached questions, kept in sync)
