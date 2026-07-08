<!--
  Prompt for the auto-maintain workflow (issue → code).
  Edit this file to change how the agent implements issues — no YAML needed.

  Runtime variables (exported into the shell by the workflow):
    $ISSUE_NUMBER  the GitHub issue to implement
  The workflow already created your branch from the latest main and will
  format, commit, push and open the PR for you.
-->
Implement GitHub issue #$ISSUE_NUMBER in this Rust crate.

IMPORTANT: your branch already exists (created from the latest `main`), and the
workflow will format, commit, push and open the pull request for you. So do NOT
create branches, run git, commit, push, or open a PR. Only edit files in the
working tree and leave your changes uncommitted.

Steps:
1. Read the issue: `gh issue view "$ISSUE_NUMBER"`.
2. Read `CLAUDE.md` and the relevant `docs/` sections for conventions. Honour the
   functional-core rules (immutability, pure planner/tools, no panics, no network
   in the core).
3. Work test-first (TDD): write the failing test(s) in `tests/` that encode the
   acceptance criteria, run `cargo test` and SEE THEM FAIL, then implement the
   minimum under `src/` until green.
4. Self-check: `cargo build --all-targets`,
   `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`.
   (Formatting is applied by the workflow — no need to run `cargo fmt`.)
5. If you implemented a documented capability, flip its entry in
   `docs/02-agent-harness.md` from 🚧 Planned to ✅ Implemented.

Leave ALL changes uncommitted in the working tree.
