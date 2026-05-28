# Sensitivity and Export Rules

Respect each Durable Record's `sensitivity` metadata before using or exporting its contents.

Sensitivity values:

- `public`: Plaintext may be committed. Include in ChatGPT Memory Digests by default.
- `personal`: Plaintext may be committed to the private repository. Include in ChatGPT Memory Digests by default.
- `restricted`: Do not commit plaintext. Exclude from ChatGPT Memory Digests by default.

Do not export restricted information to ChatGPT, generated Context Packs, ChatGPT Memory Digests, or other distribution targets unless the user explicitly instructs you to do so.

Sensitivity is behavior metadata, not folder structure. Do not infer sensitivity from a record's path alone.
