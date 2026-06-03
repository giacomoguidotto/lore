---
status: accepted
---

# Keep Mneme Discoverable But Not Preloaded

Mneme should be available to agents through global or other-project instructions, but broad Mneme context should not be preloaded by default.

Notion is the source of truth. Mneme exists so agents can understand, export, validate, and package Notion-backed knowledge without loading the whole workspace into every task.

The discovery rule does not belong in this repo's `AGENTS.md`; that file is for agents already working inside Mneme.

## Consequences

External agents should search Notion first when they need Giacomo's durable personal context. They should consult Mneme only when they need repo-local policy, Notion export/schema infrastructure, migration or historical support records, generated context packs, or a local fallback when Notion is unavailable.

When Mneme is consulted, agents should narrow-load relevant docs, exports, or support records. Generated artifacts remain on-demand outputs, not default context.
