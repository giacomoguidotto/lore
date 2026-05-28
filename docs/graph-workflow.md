# Graph Workflow

Graph artifacts are generated outputs derived from Durable Records. They are not
canonical source and are not loaded by default by agents.

## V1 Policy

- Graph artifacts are generated under `dist/graph/`.
- Graph nodes come from Durable Records under `records/active/` and
  `records/archived/`.
- Graph edges are inferred only from explicit Stable ID references in Markdown
  bodies.
- V1 does not support typed relationships.
- V1 does not require explicit graph metadata in Record Contract frontmatter.

Typed relationships, alias handling, graph visualization, and richer inference
are deferred until real records prove the need.
