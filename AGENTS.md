# Agent Instructions

Mneme is Giacomo's local-first personal context system for durable personal knowledge, preferences, history, projects, and current state.

Use Mneme only when the task depends on Giacomo's durable personal context; otherwise, do not load it.

When Mneme is relevant, start with [agent navigation](docs/agents/navigation.md).

Use Rust through `mise`; validate records with:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

More narrow instructions:

- [Sensitivity and export rules](docs/agents/sensitivity.md)
- [Source-of-truth rules](docs/agents/source-of-truth.md)
- [Development workflow](docs/agents/development.md)
