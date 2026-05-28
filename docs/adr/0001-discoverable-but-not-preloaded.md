---
status: accepted
---

# Keep Mneme Discoverable But Not Preloaded

Mneme should be available to agents through a concise global discovery instruction, but agents should not load broad Mneme context by default. This preserves context-window hygiene while still allowing durable personal knowledge to improve answers when the task actually depends on Giacomo's preferences, history, current state, or ongoing projects.

## Considered Options

- Always preload a generated Mneme context pack.
- Require every project to opt into Mneme locally.
- Use a global discovery instruction with narrow loading.

## Consequences

Agents must decide whether Mneme is relevant before inspecting it, and then load only the records needed for the task. Generated distribution artifacts remain on-demand outputs, not default context.
