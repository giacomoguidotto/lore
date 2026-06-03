---
id: rec_bee67d8e93
title: Brainboard Notion Task System
summary: Notes about Giacomo's canonical Brainboard Notion workspace, its Tasks database, and the current planning-model mismatch to address before automation.
tags:
  - notion
  - productivity
  - task-system
  - strategy
  - sync
status: active
sensitivity: personal
created: 2026-06-03
updated: 2026-06-03
source: conversation
---

Captured from a conversation on 2026-06-03. Do not modify the Notion workspace unless Giacomo explicitly asks for a concrete Notion change.

## Canonical System

- Brainboard is Giacomo's canonical Notion workspace for life-planning notes and tasks.
- The Brainboard workspace contains a Tasks database that currently tracks Giacomo's task lanes and sub-items.
- Mneme supports Brainboard by documenting schema decisions, keeping migration/export artifacts, and helping agents reason over Notion-derived context.
- Mneme must not diverge from Brainboard; if Notion and a local support artifact disagree, prefer Notion unless Giacomo identifies the local artifact as the freshest migration source.

## Observed Tasks Database Shape

- The Tasks database has a `Priority` select with values including Critical, High, Medium, and Low.
- The Tasks database has a `Focus` status with values including Signal, Noise, Wait, To do, and Done.
- The Tasks database has parent/subtask relations and uses the six-lane planning taxonomy documented in rec_e0722701cd.

## Known Misalignment

- `Focus` currently mixes strategic role and execution state.
- `Signal` and `Noise` answer whether a task matters strategically.
- `Wait`, `To do`, and `Done` answer where a task sits operationally.
- The missing middle category is `Foundation`: work that does not directly compound career leverage, revenue, product validation, or distribution, but protects the system that makes those possible.

## Examples From The Current Model

- `advance`, `build`, and `network` are high-priority signal lanes.
- `upskill` is a medium-priority signal lane.
- `respect your body` is better understood as foundation rather than ordinary signal or noise.
- `love` should not be treated as the same kind of noise as errands or cleanup tasks.
- `earn` being high-priority but marked noise conflicts with Giacomo's stated revenue-optimization model.
- Build projects can belong to a signal lane while still requiring one active bet to be chosen at a time.

## Future Infrastructure Direction

- Before automating exports or restructuring Notion, separate strategic classification from workflow state.
- Candidate strategic classification: Signal, Foundation, Noise.
- Candidate workflow state: To do, Active, Wait, Done.
- Keep any future Notion migration non-destructive until the model has been reviewed.
