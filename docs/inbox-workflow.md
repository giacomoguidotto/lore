# Inbox Workflow

The Inbox is the low-friction intake path for raw material that may help grow
Mneme. Inbox material is not canonical and does not follow the Record Contract.

## V1 Flow

- Raw material is dumped into ignored `inbox/`.
- ChatGPT-learned context is exported into `inbox/` before review. See
  `docs/chatgpt-intake.md`.
- A scheduled agent routine may process `inbox/` on a branch.
- The agent turns useful inbox material into Durable Records under
  `records/active/` or `records/archived/`.
- Durable Records promoted from inbox material use `source: import`.
- The same branch removes the inbox material that was consumed.
- Giacomo reviews the proposed Durable Records and inbox removals in a GitHub
  pull request.
- Inbox material that is not promoted may remain in `inbox/` for later review or
  be deleted.

This workflow keeps capture fast while making durability a reviewed act.
