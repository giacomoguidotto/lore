# Source-of-Truth Rules

Prefer updating source Durable Records over editing generated distribution artifacts.

Source files:

- `CONTEXT.md` defines Mneme's Shared Context.
- `records/active/` contains current committed Durable Records.
- `records/archived/` contains retained history excluded from default retrieval.
- `restricted/` is local-only unless an explicit encrypted workflow says otherwise.

Generated outputs are not source of truth:

- `dist/`
- Context Packs
- ChatGPT Memory Digests
- graph exports
- reports

Treat generated artifacts as outputs that can be regenerated from source records.
