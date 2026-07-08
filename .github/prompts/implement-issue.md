<!--
  Prompt for auto-maintain (issue → code). Edit freely; no YAML.
  Var: $ISSUE_NUMBER. The workflow branches, formats, commits, pushes and opens
  the PR for you — you ONLY edit code and make the checks pass.
-->
Implement GitHub issue #$ISSUE_NUMBER in this functional-core Rust crate.

The workflow handles all git/PR steps. Do NOT run git, commit, push, open a PR,
or run `cargo fmt`. Just edit files and make the checks pass.

1. `gh issue view "$ISSUE_NUMBER"` — read the spec + acceptance criteria.
2. Skim `CLAUDE.md`. Code lives in `src/{agent,harness,tool,message}.rs`; tests
   in `tests/`. Keep it pure (immutable, no panics, no I/O in the core).
3. Implement test-first: add the test(s) in `tests/`, then the code in `src/`.
   If the issue maps to a capability, flip it 🚧→✅ in `docs/02-agent-harness.md`.
4. Validate — this is the whole job. Run:
   `cargo build --all-targets && cargo clippy --all-targets -- -D warnings && cargo test --all-targets`
   Fix and re-run until it exits 0. These are the exact checks `ci.yml` runs, so
   green here == green on the PR. Do not stop until it passes.

Then stop — leave your changes uncommitted. The workflow takes it from there.
