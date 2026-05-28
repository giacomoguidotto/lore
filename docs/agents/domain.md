# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root.
- **`docs/adr/`** for ADRs that touch the area about to be changed.
- **`AGENTS.md`** for Mneme-specific loading and sensitivity rules.

If any of these files don't exist, proceed silently. Don't flag their absence or suggest creating them upfront. Producer skills create them lazily when terms or decisions get resolved.

## Layout

This is a single-context repo:

```text
/
├── CONTEXT.md
├── docs/adr/
└── docs/agents/
```

There is no `CONTEXT-MAP.md` at the root. Treat root `CONTEXT.md` as the shared glossary for Mneme unless a future context map is added.

## Use the glossary's vocabulary

When output names a domain concept in an issue title, refactor proposal, hypothesis, or test name, use the term as defined in `CONTEXT.md`. Do not drift to synonyms the glossary explicitly avoids.

If the concept needed is not in the glossary yet, either reconsider the language or note the gap for `grill-with-docs`.

## Respect Mneme's narrow-load rule

Mneme is Giacomo's canonical personal context repository. Use it only when the task depends on durable personal context, preferences, history, current state, or ongoing projects.

When Mneme is relevant, read `CONTEXT.md` first, then load only the narrow records needed for the current task. Respect each record's sensitivity metadata, treat generated artifacts as outputs rather than source, and do not inspect `inbox/` unless explicitly asked.

Prefer updating source durable records over editing generated distribution artifacts.

## Flag ADR conflicts

If output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> Contradicts ADR-0001 (discoverable but not preloaded), but worth reopening because...
