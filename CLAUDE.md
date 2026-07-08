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

## Workflow: TDD, always

Every change follows red → green → refactor:

1. **Red.** Write the failing test(s) that encode the acceptance criteria first.
2. **Green.** Write the minimum code to make them pass.
3. **Refactor.** Clean up with the tests still green.

Never write implementation before its test. Tests live in `tests/` (or
`#[cfg(test)]` units).

## Definition of done for an issue

**All of these must pass locally before you open the PR — do not open a PR if
any fails:**

1. Behaviour matches the referenced `docs/` acceptance criteria.
2. `cargo fmt --all` — code is formatted.
3. `cargo build --all-targets` — the crate builds.
4. `cargo clippy --all-targets -- -D warnings` — no warnings.
5. `cargo test --all-targets` — all tests pass, and they were written test-first.
6. If you added a capability, update the matching entry in
   `docs/02-agent-harness.md` from 🚧 Planned to ✅ Implemented **in the same PR**
   so the backlog generator stops re-filing it.

CI enforces steps 2–5 on every PR; a PR that fails CI will not be merged.

## Git / PR conventions

- One issue per PR. Keep diffs small and reviewable.
- **In the automated (auto-maintain) path, the workflow — not the agent — owns
  git.** It branches `auto/issue-<n>` from the latest `main`, runs the formatter,
  commits, pushes and opens the PR. The agent must therefore **only edit files
  and leave the working tree uncommitted** — do not create branches, run git,
  commit, push, or open PRs.
- Human contributors: branch `auto/issue-<n>`, PR body starts with `Closes #<n>`.
- Never merge. Never force-push to `main`. Never edit workflow files or secrets
  unless the issue is explicitly about the automation.

## Dependencies

The crate has **zero** third-party dependencies by design. Do not add one unless
an issue explicitly calls for it (e.g. `serde` for C7); if you must, justify it
in the PR body.
