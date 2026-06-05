---
name: dump-knowledge
description: Prepare and apply Notion knowledge updates from completed agent sessions by inspecting live Notion structure, extracting session facts, and drafting exact target writes for user approval. Use when the user says /dump-knowledge, /update-knowledge, asks to dump/save/migrate session knowledge to Notion, or finishes an agent session.
---

# Dump Knowledge

## Quick Start

After an agent session, inspect Notion live, extract durable knowledge from the session, show Giacomo exactly what would be written where, and wait for approval before writing.

## Workflow

1. Inspect current context:
   - Current repo, branch, changed files, commits, command outputs, and conversation decisions.
   - Any user-provided notes or final summary.
   - Existing Notion pages/databases that look relevant.

2. Inspect Notion live:
   - Use the available Notion connector/tooling.
   - Read the relevant databases/pages directly from Notion at runtime.
   - Do not rely on committed docs or old snapshots.

3. Extract only durable knowledge:
   - Progress made.
   - Decisions and rationale.
   - Open tasks, blockers, and follow-ups.
   - Links to repos, branches, commits, PRs, issues, or artifacts.
   - User preferences or personal context only when explicitly stated.

4. Produce a migration draft:
   - Target Notion database/page.
   - New page vs update existing page.
   - Property/value mapping, using the live Notion fields.
   - Page body draft.
   - Relations or links to add.
   - Items intentionally skipped.
   - Ambiguities/questions.

5. Ask for confirmation:
   - Do not write anything to Notion until Giacomo approves the draft.
   - If the mapping is ambiguous, ask before writing.

6. Write and verify:
   - Apply only the approved changes.
   - Read back the updated Notion item.
   - Report what changed and what remains unresolved.

## Draft Format

```md
## Proposed Notion Update

Target: <database/page>
Action: <create/update>

Fields:
- <field>: <value>

Body:
<draft content>

Links/relations:
- <target>

Skipped:
- <item and reason>

Questions:
- <question, if any>
```

## Rules

- Notion is the source of truth.
- Never invent facts to fit a Notion field.
- Never write before explicit approval.
- Prefer one clear update over broad context dumps.
- If Notion access is unavailable, produce only a draft and say it was not verified against live Notion.
