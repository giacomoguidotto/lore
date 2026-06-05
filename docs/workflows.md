# Workflows

Lore defines workflows; external runtimes execute them. Notion remains the source of truth.

## 1. Capture

Trigger: manual Codex command, starting with `/dump-knowledge`; `/update-knowledge` may remain an alias.

Flow: use `skills/dump-knowledge`; inspect the live target Notion database; summarize the coding session's progress, decisions, tasks, blockers, and follow-ups; ask Giacomo to confirm the draft; write the approved update to Notion.

Rule: do not write session knowledge to Notion without explicit confirmation.

Rule: if the live Notion database cannot represent the update without ambiguity, ask before writing.

Interactive life-context capture uses `skills/grill-knowledge`: grill one question at a time, produce a Notion draft at the end, and write only after confirmation.

## 2. Sync

Trigger: scheduled agent run, or manual run.

Flow: read the Notion diff since the last accepted snapshot; detect incomplete or ambiguous state; create Clarification Requests through the configured medium such as Telegram or Slack; wait for answers when needed; write the resulting snapshot, diff, and context packs to this repo; validate; open a PR that requires Giacomo's approval.

Rule: Sync may run even when there is no Notion diff if unresolved Clarification Requests or ambiguous states exist.

## 2a. Codex Sync

Codex Sync is a no-op. Codex should follow its instructions, consult Lore only when needed, and narrow-load the repo's validated context packs or support docs.

## 2b. ChatGPT Sync

Trigger: after a successful Sync.

Flow: generate a short ChatGPT Memory Digest; use a browser-capable agent to open ChatGPT; paste the digest with instructions to update memory.

Rule: the digest must be routing-safe and high-level. It should teach ChatGPT where the source of truth lives and what preferences matter, not copy broad Notion content.

## 3. Portfolio

Trigger: manual or scheduled generation, after the portfolio architecture is designed.

Flow: use Notion-backed profile, project, and career facts to generate portfolio data or UI.

Rule: the portfolio must not invent claims. Whether the portfolio fetches from Lore, Notion, or a generated artifact is unresolved and needs a dedicated architecture pass.

## 4. Govern

Trigger: rare scheduled sanity check, or manual run.

Flow: inspect Notion and repo artifacts for missing information, permission drift, sensitivity mistakes, stale context packs, or impossible states; create Clarification Requests when needed; update Notion content only after Giacomo approves the concrete change.

Rule: Govern is for holes and drift, not routine syncing.

## Non-Workflows

- Initial migration from old support records into Notion is a one-time project.
- Runtime setup for Telegram, Slack, OpenClaw, browser automation, cron, or CI is implementation detail.
- Lore should not run background processes by itself.
