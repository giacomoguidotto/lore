# Lore

## Glossary

**Lore**: Giacomo's Notion Knowledge Gateway.
_Avoid_: second brain, source of truth

**Notion Knowledge Gateway**: A local repo that makes Giacomo's canonical Notion workspace legible, auditable, and usable by agents.
_Avoid_: personal knowledge base, memory store

**Notion Source**: The canonical workspace for Giacomo's personal knowledge, tasks, projects, and portfolio facts.

**Support Record**: A Markdown artifact used as a migration seed, historical snapshot, or Notion-derived retrieval aid.

**Record Contract**: The required frontmatter for every support record.

**Workflow**: The casual umbrella name for an agent-enabled process described by an Automation Definition.
_Avoid_: automation run, background job

**Automation Definition**: A repo-owned description of an agent workflow's purpose, inputs, outputs, permissions, and validation rules.
_Avoid_: automation run, source of truth

**Automation Runtime**: The external system that executes an automation definition, such as Codex, a scheduled agent, or Notion automation.
_Avoid_: Lore-owned background process

**Clarification Request**: A question created when Notion state is incomplete, ambiguous, or unsafe to sync without Giacomo's answer.
_Avoid_: notification, reminder

**Stable ID**: A permanent `rec_...` identifier that survives renames, moves, and title changes.

**Narrow Load**: Read only the records relevant to the current task.

**Sensitivity**: Record behavior policy: `public`, `personal`, or `restricted`.

**Inbox**: Ignored raw intake. It is not canonical and is not validated.

**Restricted Record**: A support record whose plaintext must stay outside committed `records/`.

**Archived Record**: A support record kept for history and excluded from default retrieval.

**Distribution Artifact**: A generated output under `dist/`. It is never source of truth.

**ChatGPT Memory Digest**: A reviewable generated artifact for transferring routing-safe summaries into ChatGPT or Codex memory.
