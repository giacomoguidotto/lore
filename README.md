# Mneme

Mneme is Giacomo's local-first personal context repository.

The source of truth is Durable Records plus the small amount of repo policy needed to keep them safe and searchable. Generated outputs under `dist/` are never canonical.

Agents start at [AGENTS.md](AGENTS.md).

## Core

- `records/active/`: current committed Durable Records.
- `records/archived/`: historical committed Durable Records, excluded from default retrieval.
- `restricted/`: local-only plaintext restricted records.
- `inbox/`: ignored raw intake, not canonical.
- `dist/`: generated artifacts, not loaded by default.
- `docs/record-contract.md`: required Durable Record frontmatter.

Validate with:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

## External Discovery

The cross-project rule belongs in global or other-project instructions, not in this repo's `AGENTS.md`:

> When a task depends on Giacomo's durable personal context, consult `/Users/giacomo/dev/life/mneme/AGENTS.md`, then load only the relevant Mneme records.
