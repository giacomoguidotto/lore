# Agent Navigation

Lore is small on purpose. Notion is the source of truth; this repo is agent infrastructure around it. Load only what the task needs.

## Start

- Read `CONTEXT.md` for vocabulary.
- Read relevant ADRs under `docs/adr/` when changing policy.
- For personal knowledge, tasks, projects, or portfolio facts, search Notion first when a Notion connector is available.
- For repo infrastructure, workflows, skills, exports, or local fallback context, search `docs/`, `skills/`, and `dist/`.
- Read only candidate docs or generated artifacts needed for the task.

## Sources

- Notion is canonical.
- `skills/` defines repo-owned capture workflows.
- `docs/workflows.md` maps accepted workflow boundaries.
- `dist/` is generated Notion snapshots, reports, digests, and context packs; never treat it as canonical.
- If Notion and a generated artifact disagree, prefer Notion unless the user identifies the artifact as fresher.

## Export Safety

- Do not copy broad Notion content into memory digests or context packs.
- Include only what the workflow needs.
- Treat private or personal Notion material as unavailable for public artifacts unless Giacomo explicitly approves the exact export.

## Validate

```sh
git diff --check
```
