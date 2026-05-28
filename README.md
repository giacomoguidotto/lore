# Mneme

Mneme is Giacomo's local-first personal context system: a structured, private knowledge base for durable facts, preferences, history, projects, and current experience.

The canonical source lives in this repository. Assistant-facing outputs are generated on demand and are never the source of truth.

## Navigation

- `CONTEXT.md` defines Mneme's shared language.
- `AGENTS.md` defines how agents should inspect Mneme.
- `docs/record-contract.md` defines required Durable Record metadata.
- `docs/folder-structure.md` defines the initial Durable Record layout.
- `docs/inbox-workflow.md` defines raw intake and promotion.
- `docs/chatgpt-intake.md` defines ChatGPT-to-Mneme intake.
- `docs/local-restore.md` defines ignored local folder restoration.
- `docs/distribution-artifacts.md` defines generated artifact policy.
- `docs/digest-workflow.md` defines ChatGPT Memory Digest generation.
- `docs/graph-workflow.md` defines graph artifact generation.
- `docs/validation.md` defines Durable Record validation.
- `docs/v1-scope.md` defines the first usable milestone.
- `docs/adr/` records non-obvious decisions when they are hard to reverse.

## Core Rules

- Load narrowly: inspect only records relevant to the current task.
- Keep sensitivity as metadata, not folder structure.
- Use stable IDs for durable records.
- Generate graph and distribution artifacts from source records.
- Treat ChatGPT memory as a downstream digest, not canonical storage.
