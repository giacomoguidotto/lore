# Record Contract

The Record Contract defines the required metadata for every Durable Record in
Mneme. It is the safety and retrieval spine for the repository; folder paths
support human navigation, but policy and assistant behavior are driven by this
contract.

## Settled Decisions

- Define the Record Contract before the initial Durable Record folder structure.
- Store Durable Records as Markdown files.
- Express each Durable Record's Record Contract as YAML frontmatter.
- Give each Durable Record an opaque, permanent Stable ID that is not derived
  from the filename, title, folder, or record content.
- Format Stable IDs as `rec_` followed by a lowercase random ULID-style suffix.
- Use human-readable filenames for navigation; filenames are not identity.
- Use `summary` and `tags` for the informational aspect of a Durable Record.
- Require the `tags` field, but allow it to be an empty list.
- Do not require a single `type` enum.
- Use `active` and `archived` as the initial lifecycle statuses.
- Exclude archived records from default assistant retrieval and generated
  Context Packs unless explicitly requested.
- Use `sensitivity` as the single required behavior-policy field for v1.
- Do not require separate `storage` or `sync` fields in v1.
- Require `created`, `updated`, and `source` fields for lightweight
  provenance.
- Do not require explicit graph relationship metadata in v1.
- Require every v1 Record Contract field listed below. Additional fields are
  outside the required v1 contract until real records prove they are needed.

## Frontmatter

Each Durable Record begins with YAML frontmatter, followed by human-readable
Markdown content.

```yaml
---
id: rec_01jz8w2k9f6p4b7q3x5n8a1c0d
title: Example durable record
summary: Short durable summary of the record.
tags:
  - example
  - profile
status: active
sensitivity: personal
created: 2026-05-28
updated: 2026-05-28
source: manual
---
```

All fields shown above are required in v1.

## Required Fields

### `id`

The Stable ID for the Durable Record.

### `title`

A human-readable title for navigation and review. Titles may change without
changing the Stable ID.

### `summary`

A short retrieval-facing description of the Durable Record. Agents may use the
summary to decide whether the full record is relevant to the current task.

### `tags`

Keywords that describe the information captured by the Durable Record. Tags
combine with the summary to support retrieval, review, graph generation, and
Context Pack generation. Tags do not drive behavior policy. The field is
required, but may be an empty list.

### `status`

The lifecycle state of the Durable Record. Status describes whether the record
should currently be trusted, used, reviewed, or retired.

Allowed values:

- `active`: Current and usable as durable context.
- `archived`: Retained for history, but excluded from default assistant
  retrieval and generated Context Packs unless explicitly requested.

### `sensitivity`

The behavior-policy field for the Durable Record. Sensitivity determines default
Git storage and ChatGPT Memory Digest handling.

Allowed values:

- `public`: Plaintext may be committed. Include in ChatGPT Memory Digests by
  default.
- `personal`: Plaintext may be committed to the private repository. Include in
  ChatGPT Memory Digests by default.
- `restricted`: Do not commit plaintext. Exclude from ChatGPT Memory Digests by
  default.

ChatGPT Memory Digest inclusion means summarized or digested transfer, not raw
full-record sync by default.

Restricted records may be stored locally as plaintext or committed as
ciphertext, but that mechanism is decided by later restore and backup workflows.

Sensitivity does not determine folder location by itself.

### `created`

The date the Durable Record was first created.

### `updated`

The date the Durable Record's durable meaning last changed. Formatting-only
edits do not need to change `updated`.

### `source`

How the Durable Record entered Mneme.

Allowed values:

- `manual`: Written deliberately by Giacomo or an assisting agent.
- `import`: Created from reviewed Inbox or external material.
- `conversation`: Captured from a conversation.
- `generated`: Produced by a generation workflow and accepted as a Durable
  Record.

## Graph Metadata

The v1 Record Contract does not require explicit graph relationship metadata.
Graph generation may infer simple edges from Stable ID references, titles,
summaries, tags, and Markdown content. Typed relationships can be added later if
real records prove they are useful.

See `docs/graph-workflow.md`.

## Validation

The v1 Record Contract is enforced by the Rust `validate-records` binary. See
`docs/validation.md`.

## Stable IDs

Stable IDs remain valid across file renames, title changes, folder moves,
content changes, exports, graph indexing, assistant references, and migrations.

Stable IDs use the form `rec_<suffix>`, where `<suffix>` is a lowercase random
ULID-style value. The suffix must not encode the title, folder, person, project,
or topic described by the record.

Filenames should be human-readable because Stable IDs provide identity. A file
may be renamed without changing the Durable Record's identity.
