---
name: grill-knowledge
description: Interview the user about life events, decisions, or personal context until the facts are clear, then draft a Notion write plan without writing until approved. Use when the user says /grill-knowledge, wants to capture something from life, wants to be grilled before saving knowledge, or discusses personal context that may belong in Notion.
---

# Grill Knowledge

## Quick Start

Interview Giacomo one question at a time, clarify the story or decision, then draft the exact Notion update. Do not write to Notion during the grilling.

## Workflow

1. Establish the topic:
   - What happened?
   - Why does it matter?
   - Is it a task, project, decision, application, relationship, preference, profile fact, or reflection?

2. Inspect Notion only for context:
   - Search relevant existing Notion pages/databases when available.
   - Use Notion to avoid duplicates and understand where the update might belong.
   - Do not rely on committed docs or old snapshots.

3. Grill one question at a time:
   - Ask the most important unresolved question.
   - Provide your recommended answer or framing when helpful.
   - Stop asking when the remaining uncertainty would not change the Notion draft.
   - Do not write, update, or create Notion pages during the interview.

4. Produce the final draft:
   - Target Notion database/page.
   - New page vs update existing page.
   - Field/value mapping, using the live Notion fields.
   - Page body draft.
   - Links or relations to add.
   - What is intentionally not being captured.
   - Open questions, if any.

5. Ask for confirmation:
   - Write only after Giacomo explicitly approves.
   - If approval is not given, leave the draft in the chat only.

6. Write and verify when approved:
   - Apply only the approved draft.
   - Read back the Notion item.
   - Summarize what changed.

## Question Style

- Keep it tight: one question at a time.
- Preserve Giacomo's wording when it carries meaning.
- Challenge vague terms when they affect where the knowledge goes.
- Ask before turning subjective models into advice or stronger claims.

## Draft Format

```md
## Proposed Notion Capture

Topic: <short label>
Target: <database/page>
Action: <create/update>

Fields:
- <field>: <value>

Body:
<draft content>

Not capturing:
- <item and reason>

Confirm?
```

## Rules

- Notion is the source of truth.
- No Notion writes while grilling.
- No write without explicit confirmation.
- Do not hallucinate missing facts.
- If Notion access is unavailable, produce only a draft and say it was not verified against live Notion.
