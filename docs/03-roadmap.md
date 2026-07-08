# 03 · Roadmap

Phases are ordered but each capability is shippable on its own.

## Phase 1 — Functional core ✅
C1–C4. Conversation, pure tools, planner/agent, bounded loop. Done.

## Phase 2 — Richer decisions 🚧
C5 (multi-tool), C6 (retry policy). Make a single agent turn do more without
losing determinism.

## Phase 3 — Typed & observable 🚧
C7 (structured I/O), C8 (usage accounting), C9 (transcript rendering). Make runs
easy to inspect and put into PR descriptions.

## Phase 4 — Real provider 🚧
C10 (provider trait). Plug a real LLM in behind the planner seam, offline demo
still intact.

## Non-goals

- A production agent framework. This stays small on purpose.
- Auto-merging PRs. Humans always review.
- Persisting conversations to a database. In-memory only.
- Concurrency / async in the core. The loop is a synchronous fold.
