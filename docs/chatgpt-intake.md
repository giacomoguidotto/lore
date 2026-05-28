# ChatGPT Intake

ChatGPT-learned context enters Mneme through the Inbox. ChatGPT is an input
source, not a source of truth.

## V1 Flow

- Ask ChatGPT to export or summarize durable context with the prompt below.
- Save the response into `inbox/`.
- Process it through the normal Inbox Workflow.
- Promote useful material into Durable Records only after review.

## Generic Export Prompt

Use this prompt in an existing conversation, or in a new conversation where
ChatGPT has access to previous chat context or exported chat material:

```text
Export durable context you know about me that may be useful for future AI assistants.

Focus on stable or meaningfully recurring information, not temporary details.
Preserve my words verbatim where possible, especially for explicit instructions,
preferences, corrections, and personal framing.

## Categories

Output in this order:

1. Instructions: Rules I explicitly asked you to follow going forward, including
   tone, format, style, "always do X", "never do Y", and corrections to your
   behavior.
2. Identity: Name, location, languages, relationships, interests, and other
   durable personal context.
3. Career: Roles, companies, skills, work history, and professional context.
4. Projects: Projects I meaningfully built, used, maintained, or committed to.
   Prefer one entry per project. Include purpose, current status, and key
   decisions when known.
5. Preferences: Opinions, tastes, defaults, and working-style preferences that
   apply broadly.
6. Other Context: Durable context that does not fit the categories above.

## Format

Use section headers for each category.
Within each category, list one entry per line.
Sort entries by oldest known date first.
Format each line as:

[YYYY-MM-DD] - Entry content here.

If no date is known, use [unknown].
If you are uncertain, say so inside the entry.
Do not invent facts.

## Output

Wrap the entire export in a single fenced code block for easy copying.
After the code block, state whether this appears complete or whether more
context may remain.
```

The exported result is raw intake. It does not need to satisfy the Record
Contract until reviewed and promoted into `records/`.
