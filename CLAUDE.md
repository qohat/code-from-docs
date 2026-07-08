# CLAUDE.md — conventions for the coding agent

This file is read by `auto-maintain.yml` before implementing any issue. Follow
it exactly; PRs that violate it will be rejected in review.

## What this repo is

A demo of doc-driven code generation. `docs/` is the spec. `src/` is a tiny
functional-core agent harness (a Rust crate). See `docs/01-architecture.md`.

## Rust style (non-negotiable)

- **Functional core.** No global mutable state, no `unsafe`, no panics on the
  happy or error path. Model "mutation" as functions returning new values
  (`Conversation::with`, `Toolbox::with`).
- **The planner is the only LLM seam.** Keep `agent`, `harness`, `tool`,
  `message` free of network and filesystem access. I/O belongs in `main.rs` or
  new edge modules only.
- **Errors are values.** Return `Result` with `ToolError`-style enums; never
  `unwrap`/`expect` outside tests.
- **Immutability first.** Prefer `&self` methods that return new values over
  `&mut self`.
- Match the surrounding naming, doc-comment density, and module layout.

## Definition of done for an issue

1. Behaviour matches the referenced `docs/` acceptance criteria.
2. `cargo test` passes.
3. `cargo clippy --all-targets -- -D warnings` passes.
4. New behaviour has tests in `tests/` (or `#[cfg(test)]` unit tests).
5. If you added a capability, update the matching entry in
   `docs/02-agent-harness.md` from 🚧 Planned to ✅ Implemented **in the same PR**
   so the backlog generator stops re-filing it.

## Git / PR conventions

- Branch: `auto/issue-<n>`.
- One issue per PR. Keep diffs small and reviewable.
- PR body starts with `Closes #<n>`, then a summary and testing notes.
- Never merge. Never force-push to `main`. Never edit workflow files or secrets
  unless the issue is explicitly about the automation.

## Dependencies

The crate has **zero** third-party dependencies by design. Do not add one unless
an issue explicitly calls for it (e.g. `serde` for C7); if you must, justify it
in the PR body.
