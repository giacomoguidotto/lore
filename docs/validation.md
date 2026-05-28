# Validation

Mneme validates Durable Records with:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

The validator enforces the v1 Record Contract and folder invariants:

- committed Durable Records live under `records/active/` or
  `records/archived/`;
- the folder matches `status`;
- `sensitivity: restricted` does not appear under committed `records/`;
- all required fields exist;
- enum fields use allowed values;
- Stable IDs match `rec_[a-z0-9]+` and are unique;
- `created` and `updated` use `YYYY-MM-DD`;
- `updated` is not earlier than `created`.

## Pre-Commit Hook

The validator is implemented as a Rust binary at
`tools/validate-records/main.rs`. The repository pins Rust in `.mise.toml`.

Install the toolchain with:

```sh
mise install
```

The repository includes `.githooks/pre-commit`, which runs the Rust validator
before commits. Enable it in a clone with:

```sh
git config core.hooksPath .githooks
```
