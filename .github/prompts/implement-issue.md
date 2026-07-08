<!--
  Prompt for the auto-maintain workflow (issue → code → PR).
  Edit this file to change how the agent implements issues — no YAML needed.

  Runtime variable (substituted before the run):
    $ISSUE_NUMBER   the GitHub issue to implement
  Your branch (auto/issue-$ISSUE_NUMBER) is already created from the latest main.
-->
Implement GitHub issue #$ISSUE_NUMBER in this Rust crate, then open a pull request.

You are on branch `auto/issue-$ISSUE_NUMBER`, already based on the latest `main`.

Do this:
1. Read the issue: `gh issue view "$ISSUE_NUMBER"`.
2. Read `CLAUDE.md` and the relevant `docs/` sections for conventions. Honour the
   functional-core rules (immutability, pure planner/tools, no panics, no network
   in the core).
3. Implement test-first: add the test(s) in `tests/` that encode the acceptance
   criteria, then the code under `src/`. If you implemented a documented
   capability, flip its entry in `docs/02-agent-harness.md` from 🚧 Planned to
   ✅ Implemented.
4. Format your changes: `cargo fmt --all`.
5. Commit, push, and OPEN A PULL REQUEST. This is required and must ALWAYS happen
   so your work is published even if the implementation is imperfect:
   ```
   git add -A
   git commit -m "auto-maintain: implement #$ISSUE_NUMBER"
   git push -u origin "auto/issue-$ISSUE_NUMBER"
   gh pr create --base main --head "auto/issue-$ISSUE_NUMBER" \
     --title "<the issue title>" --body "Closes #$ISSUE_NUMBER"
   ```
   If a PR for this branch already exists, reuse it (skip `gh pr create`).

You do NOT need to run the build / clippy / test suite — CI (`ci.yml`) runs those
on the pull request. Focus on a correct implementation and ALWAYS leave an open
PR behind.
