---
status: accepted
---

# Keep Mneme Discoverable But Not Preloaded

Mneme should be available to agents through global or other-project instructions, but broad Mneme context should not be preloaded by default.

The discovery rule does not belong in this repo's `AGENTS.md`; that file is for agents already working inside Mneme.

## Consequences

External agents should consult Mneme only when the task depends on Giacomo's durable personal context, then narrow-load relevant records. Generated artifacts remain on-demand outputs, not default context.
