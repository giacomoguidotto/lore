# Agent Instructions

You are inside Lore, Giacomo's local infrastructure repo for his Notion knowledge system.

Notion is the source of truth for Giacomo's personal knowledge, tasks, projects, and portfolio facts. Lore supports agents with repo policy, support records, exports, validation, and generated context packs.

Start with [agent navigation](docs/agents/navigation.md). It contains the Notion-first source-of-truth, loading, sensitivity, and validation rules.

Use Rust through `mise`:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

## Agent skills

- Issues and PRDs: GitHub Issues for `giacomoguidotto/lore`; see [issue tracker](docs/agents/issue-tracker.md).
- Triage labels: use the default five-label vocabulary; see [triage labels](docs/agents/triage-labels.md).
- Domain docs: this is a single-context repo; use root `CONTEXT.md` and relevant ADRs; see [domain docs](docs/agents/domain.md).
