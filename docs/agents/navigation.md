# Agent Navigation

Lore is small on purpose. Notion is the source of truth; this repo is agent infrastructure around it. Load only what the task needs.

## Start

- Read `CONTEXT.md` for vocabulary.
- Read relevant ADRs under `docs/adr/` when changing policy.
- For personal knowledge, tasks, projects, or portfolio facts, search Notion first when a Notion connector is available.
- For repo infrastructure, migrations, exports, or local fallback context, search filenames, frontmatter, summaries, and tags before opening full records.
- Read only candidate docs, exports, or records needed for the task.

## Records

- Committed records are support artifacts, not the canonical personal knowledge store.
- `records/active/` is default retrieval for current support records.
- `records/archived/` is historical; skip unless the user asks for history.
- `restricted/` is local-only plaintext for restricted records.
- `inbox/` is ignored raw intake; do not inspect it unless explicitly asked.
- `dist/` is generated Notion snapshots, reports, digests, and context packs; never treat it as canonical.
- If Notion and a local artifact disagree, prefer Notion unless the user identifies the local artifact as the freshest migration source.

## Sensitivity

- `public`: may be committed and included in generated memory digests by default.
- `personal`: may be committed to the private repo and included in generated memory digests by default.
- `restricted`: do not commit plaintext and do not export unless the user explicitly asks.

Sensitivity is behavior metadata. Do not infer it from topic or path alone.

## Validate

```sh
mise exec -- cargo run --quiet --bin validate-records
```

Enable the hook in a clone with:

```sh
git config core.hooksPath .githooks
```
