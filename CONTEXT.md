# Mneme

## Glossary

**Mneme**: Local agent infrastructure for Giacomo's Notion knowledge system.

**Notion Source**: The canonical workspace for Giacomo's personal knowledge, tasks, projects, and portfolio facts.

**Support Record**: A Markdown artifact used as a migration seed, historical snapshot, or Notion-derived retrieval aid.

**Record Contract**: The required frontmatter for every support record.

**Stable ID**: A permanent `rec_...` identifier that survives renames, moves, and title changes.

**Narrow Load**: Read only the records relevant to the current task.

**Sensitivity**: Record behavior policy: `public`, `personal`, or `restricted`.

**Inbox**: Ignored raw intake. It is not canonical and is not validated.

**Restricted Record**: A support record whose plaintext must stay outside committed `records/`.

**Archived Record**: A support record kept for history and excluded from default retrieval.

**Distribution Artifact**: A generated output under `dist/`. It is never source of truth.

**ChatGPT Memory Digest**: A reviewable generated artifact for transferring routing-safe summaries into ChatGPT or Codex memory.
