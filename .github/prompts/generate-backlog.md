<!--
  Prompt for the generate-backlog workflow (docs → issues).
  Edit this file to change how the backlog is generated — no YAML needed.

  Runtime variables (substituted by the workflow before the run):
    $CHANGED_DOCS  space-separated list of docs to process
    $DRY_RUN       "true" to print instead of creating issues
-->
You are the backlog generator. Turn the CHANGED docs below into a GitHub issue
backlog that the auto-maintain workflow can implement.

Changed docs to process (ONLY these): $CHANGED_DOCS

Steps:
1. Read each changed doc listed above. It is the source of truth. Note
   capabilities tagged "Planned" vs "Implemented".
2. Read the Rust source under `src/` to confirm what already exists.
3. Run `gh issue list --state all --limit 200` and read the titles so you DO NOT
   create duplicates (match on the capability, not exact wording).
4. Ensure labels exist:
   `gh label create spec --color 0e8a16 --force` and
   `gh label create auto-maintain --color 1d76db --force`.
5. For each "Planned" capability in the changed docs with no matching issue,
   create ONE small, independently-implementable issue:
     - Title: a concise imperative spec (e.g. "Add retry policy to tools").
     - Body: MUST use this constant structure so every issue is consistent:
         * "## Behaviour" — what it should do (from the doc).
         * "## Acceptance criteria" — a checklist adapted from the doc.
         * "## How to build it" — the constant standard, verbatim:
             "Follow CLAUDE.md. Work test-first (TDD): write failing tests, then
              implement. Before opening the PR, `cargo fmt --all`,
              `cargo build --all-targets`, `cargo clippy --all-targets -- -D warnings`
              and `cargo test --all-targets` must all pass. Flip this capability
              🚧→✅ in docs/02-agent-harness.md in the same PR."
         * "## Source" — the doc section (e.g. "docs/02 · C6").
     - Labels: `spec` and `auto-maintain` (`gh issue create ... --label spec --label auto-maintain`).
6. If "$DRY_RUN" is "true": DO NOT create anything — print the issues you would
   create and stop.

Keep issues small and their bodies consistent. Do not modify any files.
