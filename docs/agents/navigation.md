# Agent Navigation

Mneme is small on purpose. Load only what the task needs.

## Start

- Read `CONTEXT.md` for vocabulary.
- Read relevant ADRs under `docs/adr/` when changing policy.
- For record work, search filenames, frontmatter, summaries, and tags before opening full records.
- Read only candidate records needed for the task.

## Records

- Committed records live in `records/active/` and `records/archived/`.
- `records/active/` is default retrieval.
- `records/archived/` is historical; skip unless the user asks for history.
- `restricted/` is local-only plaintext for restricted records.
- `inbox/` is ignored raw intake; do not inspect it unless explicitly asked.
- `dist/` is generated output; never treat it as canonical.

## Sensitivity

- `public`: may be committed and included in ChatGPT Memory Digests by default.
- `personal`: may be committed to the private repo and included in ChatGPT Memory Digests by default.
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
