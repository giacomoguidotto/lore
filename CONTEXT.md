# Mneme

Mneme is a local-first personal context system that preserves Giacomo's durable knowledge, history, preferences, and current state so AI assistants can give better answers with minimal prior context.

## Language

**Shared Context**:
The single glossary used across Mneme while its language remains cohesive. Mneme starts with one Shared Context and only introduces a context map if distinct vocabularies emerge.
_Avoid_: bounded context, context map

**Mneme**:
The canonical personal knowledge base. It is the source of truth for durable facts, preferences, experiences, and context that should be available to AI assistants.
_Avoid_: Vitae, second brain, memory dump, archive

**Navigation Contract**:
The set of root-level documents and conventions that let an agent enter Mneme cold, understand how to explore it, and know which information is safe to use.
_Avoid_: docs, README, instructions

**Narrow Load**:
The rule that agents inspect only the Mneme records relevant to the current task instead of loading broad personal context by default.
_Avoid_: full context, preload, memory dump

**Global Discovery Instruction**:
A concise global Codex instruction that tells agents when and how to consult Mneme. It points agents to Mneme without injecting Mneme into every task.
_Avoid_: global context, always-on memory

**Durable Record**:
A knowledge item intended to survive beyond the current conversation or day. Every Durable Record has a stable ID.
_Avoid_: note, page, file

**Record Contract**:
The mandatory metadata every Durable Record must provide: stable identity, type, summary, status, sensitivity, storage rule, sync rule, and last update date.
_Avoid_: frontmatter, schema, metadata

**Stable ID**:
A permanent identifier for a Durable Record that remains valid across file renames, title changes, exports, graph indexing, and assistant references.
_Avoid_: filename, slug, title

**Sensitivity Metadata**:
Frontmatter fields that describe how a Durable Record may be stored, committed, exported, and used by assistants. Sensitivity does not determine the folder structure.
_Avoid_: private folder, secret section

**Local-Only Record**:
A Durable Record stored on Giacomo's encrypted disk but excluded from Git. It may be reconstructed after cloning through a restore workflow.
_Avoid_: untracked note, private file

**Encrypted Record**:
A Durable Record whose ciphertext may be committed while plaintext remains local-only.
_Avoid_: secret file, protected note

**Context Pack**:
A generated assistant-facing export derived from Mneme on demand. A Context Pack is not canonical, is not loaded by default, and may omit, summarize, or redact source records.
_Avoid_: source, backup, memory

**Distribution Artifact**:
A generated export intended for a specific consumer such as ChatGPT memory, a ChatGPT Project upload, a human review report, or a visual HTML report. Distribution Artifacts are not canonical and should be produced only when useful.
_Avoid_: source file, default context, required context

**ChatGPT Memory Digest**:
A generated Context Pack designed for manual or semi-automatic transfer into ChatGPT memory. It contains durable, safe summaries rather than raw records.
_Avoid_: ChatGPT sync, memory dump

**Project Import**:
A reviewed intake from another local project into Mneme. A Project Import summarizes relevant knowledge and must not blindly copy proprietary code, credentials, or project internals.
_Avoid_: scrape, mirror, clone

## Relationships

- **Mneme** currently has one **Shared Context**.
- A **Global Discovery Instruction** enables **Narrow Load**.
- **Mneme** contains zero or more **Durable Records**.
- A **Durable Record** has exactly one **Stable ID**.
- A **Durable Record** follows the **Record Contract**.
- A **Durable Record** has **Sensitivity Metadata**.
- A **Local-Only Record** is a **Durable Record** excluded from Git.
- An **Encrypted Record** is a **Durable Record** whose committed representation is ciphertext.
- A **Context Pack** is generated from **Mneme** and is never the source of truth.
- A **Distribution Artifact** is generated from **Mneme** and is never loaded by default.
- A **ChatGPT Memory Digest** is a **Context Pack** intended for ChatGPT memory.
- A **Project Import** feeds reviewed knowledge into **Mneme**.

## Example Dialogue

**Agent**: "Should I add this preference to the ChatGPT Memory Digest?"

**Giacomo**: "Only if the Sensitivity Metadata allows ChatGPT summaries."

**Agent**: "This Durable Record is marked `storage: local_only` and `sync.chatgpt: none`, so I will keep it out of the Context Pack."

**Giacomo**: "Good. The folder can stay under work because the topic is work, but the sensitivity rules decide where it can be synced."
