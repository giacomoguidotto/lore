# Record Contract

Support records are Markdown files under `records/active/` or `records/archived/` with required YAML frontmatter.

They preserve migration seeds, historical snapshots, or Notion-derived retrieval hints. Notion remains the source of truth for personal knowledge once the same fact exists there.

## Template

```md
---
id: rec_example01
title: Example
summary: One sentence that helps agents decide whether to open this record.
tags: []
status: active
sensitivity: personal
created: 2026-05-29
updated: 2026-05-29
source: manual
---
```

## Fields

- `id`: unique `rec_[a-z0-9]+`; identity never comes from filename, path, or title.
- `title`: non-empty navigation label.
- `summary`: non-empty retrieval hint for agents.
- `tags`: list of non-empty strings; `[]` is allowed.
- `status`: `active` or `archived`; must match the containing folder.
- `sensitivity`: `public`, `personal`, or `restricted`.
- `created`: `YYYY-MM-DD`.
- `updated`: `YYYY-MM-DD`; not earlier than `created`.
- `source`: `manual`, `import`, `conversation`, or `generated`.

`sensitivity: restricted` plaintext must not be committed under `records/`.

## Derived Outputs

Graphs, digests, reports, and context packs are generated into `dist/` from Notion exports and local support records. They are not canonical and are not loaded by default.

V1 graph edges come only from explicit Stable ID references in Markdown bodies. No typed relationship metadata is required.

## Validation

```sh
mise exec -- cargo run --quiet --bin validate-records
```
