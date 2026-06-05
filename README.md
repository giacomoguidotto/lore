# Lore

Lore is Giacomo's local infrastructure repo for his Notion knowledge system.

Notion is the source of truth for personal knowledge, tasks, projects, and portfolio facts. Lore keeps the workflows, skills, policy docs, and generated artifacts that make the Notion workspace usable by Codex, ChatGPT, and other agents.

Generated outputs under `dist/` are never canonical. They are reviewable artifacts derived from Notion for downstream tools.

Agents start at [AGENTS.md](AGENTS.md).

## Core

- `dist/`: generated Notion snapshots, reports, digests, and context packs.
- `skills/`: repo-owned skill definitions for Notion knowledge capture.
- `docs/workflows.md`: supported agent workflows around Notion.
- `docs/agents/`: repo-local agent navigation and issue-tracker guidance.
- `docs/adr/`: accepted policy decisions.

Validate docs and patches with:

```sh
git diff --check
```

When generated artifact validators are added, run the workflow-specific validator documented next to that artifact.

## External Discovery

The cross-project rule belongs in global or other-project instructions, not in this repo's `AGENTS.md`:

> When a task depends on Giacomo's durable personal context, search Notion first. If the agent cannot reach Notion directly or needs repo-local infrastructure, consult `/Users/giacomo/dev/life/lore/AGENTS.md`, then narrow-load only the relevant Lore docs or generated artifacts.
