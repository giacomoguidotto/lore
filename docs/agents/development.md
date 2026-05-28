# Development Workflow

Mneme uses Rust for local tooling and pins the toolchain in `.mise.toml`.

Install the toolchain with:

```sh
mise install
```

Validate Durable Records with:

```sh
mise exec -- cargo run --quiet --bin validate-records
```

The validator enforces the v1 Record Contract and folder invariants.

Enable the repository pre-commit hook with:

```sh
git config core.hooksPath .githooks
```
