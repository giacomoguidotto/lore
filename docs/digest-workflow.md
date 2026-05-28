# Digest Workflow

ChatGPT Memory Digests are generated Distribution Artifacts that make Mneme
easy to review and transfer into ChatGPT. They are not automatic sync and are
not canonical source.

## Inclusion Policy

By default, a ChatGPT Memory Digest includes only Durable Records where:

- `status: active`;
- `sensitivity: public` or `sensitivity: personal`.

By default, a ChatGPT Memory Digest excludes Durable Records where:

- `status: archived`;
- `sensitivity: restricted`.

Excluded records may be included only through an explicit review workflow.

## Output Location

ChatGPT Memory Digests are generated under `dist/digests/`.

## Format

The digest format should be easy for GPT to process and easy for Giacomo to
copy, review, and edit before transfer.

Each digest should:

- use Markdown;
- wrap the transferable digest content in a single fenced code block;
- use stable section headers;
- list one entry per line;
- prefix each entry with `[YYYY-MM-DD]` using the Durable Record's `updated`
  date, or `[unknown]` only when no date is available;
- preserve Giacomo's words verbatim where the source record explicitly captures
  instructions or preferences;
- include `id`, `title`, `summary`, `tags`, and a concise body summary;
- avoid raw full-record export by default.

## V1 Sections

Digest generators should group entries into these sections, in this order:

1. `Instructions`
2. `Identity`
3. `Career`
4. `Projects`
5. `Preferences`
6. `Other Context`

These sections are an export format, not a source taxonomy. Durable Records do
not need a `type` field to support them; generators can use summaries, tags,
titles, and content to place entries.

## Entry Shape

Within each section, entries should be sorted by oldest date first:

```text
[YYYY-MM-DD] - id: rec_...; title: Title; tags: tag-a, tag-b; summary: Summary. Details: concise generated body summary.
```

After the code block, the digest should state whether the export is complete or
whether more eligible records remain.
