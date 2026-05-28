# Mneme

Mneme is Giacomo's local-first personal context system: a structured, private knowledge base for durable facts, preferences, history, projects, and current experience.

The canonical source lives in this repository. Assistant-facing outputs are generated on demand and are never the source of truth.

## Navigation

- `CONTEXT.md` defines Mneme's shared language.
- `AGENTS.md` defines how agents should inspect Mneme.
- `docs/adr/` records non-obvious decisions when they are hard to reverse.

## Core Rules

- Load narrowly: inspect only records relevant to the current task.
- Keep sensitivity as metadata, not folder structure.
- Use stable IDs for durable records.
- Generate graph and distribution artifacts from source records.
- Treat ChatGPT memory as a downstream digest, not canonical storage.
