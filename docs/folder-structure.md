# Folder Structure

Mneme's initial folder structure optimizes for handling state, not knowledge
category. Durable Record identity comes from Stable IDs, so folders and
filenames may change without breaking references.

```text
records/
  active/
  archived/
inbox/
restricted/
dist/
docs/
```

## Rules

- `records/active/` contains plaintext `public` and `personal` Durable Records
  with `status: active`.
- `records/archived/` contains plaintext `public` and `personal` Durable
  Records with `status: archived`.
- `inbox/` contains raw unprocessed intake and is excluded from Git.
- Files in `inbox/` are not Durable Records and are not validated by the Record
  Contract.
- `inbox/` may contain public, personal, or restricted material before review.
- See `docs/inbox-workflow.md` for promotion rules.
- `restricted/` contains plaintext `restricted` Durable Records and is excluded
  from Git.
- Local-only folders commit `.gitkeep` placeholders so they exist in fresh
  clones. See `docs/local-restore.md`.
- Committed restricted ciphertext is deferred until restore and backup workflows
  prove the need.
- `dist/` contains generated Distribution Artifacts, not canonical Durable
  Records. See `docs/distribution-artifacts.md`.
- Durable Record filenames should be human-readable. Filenames are labels for
  navigation, not identity.

This structure is a v1 convention. Because Stable IDs are independent of paths,
the folder structure can change later without breaking Durable Record identity.
