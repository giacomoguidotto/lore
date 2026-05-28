# Distribution Artifacts

Distribution Artifacts are generated outputs derived from Mneme. They are never
canonical source, and agents do not load them by default.

## V1 Policy

- `dist/` contains generated artifacts only.
- `dist/` may contain committed safe artifacts when useful.
- `dist/private/` is ignored and may contain generated artifacts that should not
  be committed.
- `dist/tmp/` is ignored and may contain temporary generated artifacts.
- ChatGPT Memory Digests are generated under `dist/digests/`.
- Graph artifacts are generated under `dist/graph/`.
- Human review reports are generated under `dist/reports/`.
- No Distribution Artifact is loaded by default by agents.

See `docs/digest-workflow.md` for ChatGPT Memory Digest format and inclusion
rules.

Source Durable Records remain canonical. Distribution Artifacts can always be
regenerated from source records and review workflows.
