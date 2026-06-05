---
status: accepted
---

# Keep Lore Discoverable But Not Preloaded

Lore should be available to agents through global or other-project instructions, but broad Lore context should not be preloaded by default.

Notion is the source of truth. Lore exists so agents can understand, export, validate, and package Notion-backed knowledge without loading the whole workspace into every task.

The discovery rule does not belong in this repo's `AGENTS.md`; that file is for agents already working inside Lore.

## Consequences

External agents should search Notion first when they need Giacomo's durable personal context. They should consult Lore only when they need repo-local policy, Notion export infrastructure, generated context packs, or a local fallback when Notion is unavailable.

When Lore is consulted, agents should narrow-load relevant docs, skills, exports, or generated artifacts. Generated artifacts remain on-demand outputs, not default context.
