# Local Restore

Mneme v1 keeps local-only folder structure available in fresh clones by
committing `.gitkeep` placeholders while ignoring each folder's contents.

Tracked placeholders:

- `inbox/.gitkeep`
- `restricted/.gitkeep`
- `dist/private/.gitkeep`
- `dist/tmp/.gitkeep`

Ignored contents:

- `inbox/*`
- `restricted/*`
- `dist/private/*`
- `dist/tmp/*`

V1 does not restore local-only contents. Encrypted backup, ciphertext storage,
and private restore workflows are deferred until real usage proves the need.
