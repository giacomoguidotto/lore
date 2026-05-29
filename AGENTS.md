# Agent Instructions

You are inside Mneme, Giacomo's local-first personal context repository.

Start with [agent navigation](docs/agents/navigation.md). It contains the record loading, sensitivity, source-of-truth, and validation rules.

Use Rust through `mise`:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

## Agent skills

- Issues and PRDs: GitHub Issues for `giacomoguidotto/mneme`; see [issue tracker](docs/agents/issue-tracker.md).
- Triage labels: use the default five-label vocabulary; see [triage labels](docs/agents/triage-labels.md).
- Domain docs: this is a single-context repo; use root `CONTEXT.md` and relevant ADRs; see [domain docs](docs/agents/domain.md).
