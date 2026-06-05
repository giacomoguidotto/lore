<p align="center">
  <!-- Logo slot: add assets/logo.png, assets/logo.svg, or light/dark logo variants here. -->
</p>

<h1 align="center">Lore</h1>

<p align="center">
  <strong>Agent infrastructure for a Notion-first knowledge system.</strong><br>
  <sub>Workflows, skills, policies, and reviewable artifacts for Giacomo's Notion workspace.</sub>
</p>

<p align="center">
  <a href="https://github.com/giacomoguidotto/lore/actions"><img src="https://github.com/giacomoguidotto/lore/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/giacomoguidotto/lore/blob/main/LICENSE"><img src="https://img.shields.io/github/license/giacomoguidotto/lore" alt="License"></a>
</p>

<br>

Lore keeps agents pointed at the right source of truth. Notion owns the actual personal knowledge, tasks, projects, and portfolio facts; this repo keeps the supporting workflows and artifacts that make that workspace usable by Codex, ChatGPT, and other agents.

Generated files under `dist/` are reviewable outputs, not canonical records.

## What's Inside

- `skills/`: repo-owned agent workflows.
- `docs/workflows.md`: the accepted workflow map.
- `docs/agents/`: navigation and issue-tracker guidance for agents.
- `docs/adr/`: durable policy decisions.
- `dist/`: generated snapshots, reports, digests, and context packs.

## Principles

- Notion is canonical.
- Lore defines workflows; external runtimes execute them.
- Agents narrow-load only the docs or artifacts needed for the task.
- Public artifacts must not include private Notion material without explicit approval.

## Tooling

Lore is currently a docs, skills, and artifact repository. Validate changes with:

```sh
git diff --check
```

## Contributing

Free and open source under the [MIT License](LICENSE). See [CONTRIBUTING.md](.github/CONTRIBUTING.md) to get involved.

Agents should start at [AGENTS.md](AGENTS.md).
