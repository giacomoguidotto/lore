# Lore

Lore is Giacomo's local infrastructure repo for his Notion knowledge system.

Notion is the source of truth for personal knowledge, tasks, projects, and portfolio facts. Lore keeps the repo policy, validators, support records, exports, and generated context packs that make the Notion workspace usable by Codex, ChatGPT, and other agents.

Local records are support artifacts: migration seeds, historical snapshots, or Notion-derived retrieval aids. Generated outputs under `dist/` are never canonical.

Agents start at [AGENTS.md](AGENTS.md).

## Core

- `records/active/`: current committed support records.
- `records/archived/`: historical support records, excluded from default retrieval.
- `restricted/`: local-only plaintext restricted records.
- `inbox/`: ignored raw intake, not a source of truth.
- `dist/`: generated Notion snapshots, reports, digests, and context packs.
- `skills/`: repo-owned skill definitions for Notion knowledge capture.
- `docs/record-contract.md`: required support-record frontmatter.
- `docs/workflows.md`: supported agent workflows around Notion.

Validate with:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

## External Discovery

The cross-project rule belongs in global or other-project instructions, not in this repo's `AGENTS.md`:

> When a task depends on Giacomo's durable personal context, search Notion first. If the agent cannot reach Notion directly or needs repo-local infrastructure, consult `/Users/giacomo/dev/life/lore/AGENTS.md`, then narrow-load only the relevant Lore docs, exports, or support records.
