# Adopting code-from-docs in your own repo

This repo is a working template for a **doc-driven code-generation pipeline**:
you keep the spec in `docs/`, a workflow turns it into GitHub issues, another
implements each issue as a PR, and CI gates everything. The Rust crate here is
just the *demo target* — the pipeline itself is language-agnostic.

This guide is meta-documentation about the pipeline, so it lives at the repo
root **on purpose**: anything under `docs/` is treated as product spec and would
be turned into backlog issues.

## 1. Prerequisites

- **Install the [Claude GitHub App](https://github.com/apps/claude)** on the
  target repo. The OAuth flow needs it to mint a GitHub token via OIDC.
- **Repository secrets** (Settings → Secrets and variables → Actions):
  | Secret | Required | Source |
  |--------|----------|--------|
  | `CLAUDE_CODE_OAUTH_TOKEN` | ✅ | `claude setup-token` |
  | `DISCORD_WEBHOOK_URL` | optional | Discord → Integrations → Webhooks |
- GitHub Actions enabled. The default `GITHUB_TOKEN` may stay **read-only** —
  each workflow requests exactly the scopes it needs.

## 2. Files to copy

```
.github/workflows/
  reusable-claude.yml      # runs claude-code-action, outputs cost (copy as-is)
  reusable-notify.yml      # success/failure notification wrapper (copy as-is)
  reusable-discord.yml     # Discord notifications                (copy as-is)
  reusable-pr-comment.yml  # posts a comment on a PR              (copy as-is)
  generate-backlog.yml     # docs → issues, with memory           (copy as-is)
  docs-watch.yml           # push docs/** → dispatch backlog       (copy as-is)
  auto-maintain.yml        # issue → PR + cost comment             (adapt build tool)
  ci.yml                   # quality gate                          (rewrite for your language)
.github/prompts/
  implement-issue.md       # auto-maintain's prompt                (edit freely)
  generate-backlog.md      # generate-backlog's prompt             (edit freely)
CLAUDE.md                  # coding conventions                    (rewrite for your project)
docs/                      # your product spec                     (replace contents)
specs/                     # backlog-state.json is auto-created    (start empty)
.env.example               # documents required secret names       (copy as-is)
```

## 3. What to change per repo

1. **`ci.yml`** — replace the Rust steps (`cargo fmt` / `build` / `clippy` /
   `test`) with your stack's format + build + lint + test.
2. **Build-tool allowlist** in `auto-maintain.yml` → `claude_args` →
   `--allowedTools`: swap `Bash(cargo:*)` for `Bash(npm:*)`, `Bash(go:*)`, etc.
   Leave `Bash(gh:*)`, `Bash(git:*)`, `Read`, `Edit`, `Write` in place.
3. **`CLAUDE.md`** — your conventions and the *definition of done* the agent must
   satisfy before opening a PR.
4. **`docs/`** — your specs. Keep the tagging convention: mark each capability
   🚧 **Planned** or ✅ **Implemented**; the generator only files issues for
   Planned ones and skips Implemented ones.
4b. **`.github/prompts/*.md`** — how the agent behaves. These are plain text, so
   editing them needs no YAML. `reusable-claude` reads a `prompt_file` and
   substitutes any `session_env` vars (e.g. `$ISSUE_NUMBER`, `$CHANGED_DOCS`)
   into the text before the run — keep those `$VAR` placeholders when editing.
5. **Model** — `--model claude-sonnet-4-6` in the `claude_args` if you want a
   different tier.
6. **Labels** — `spec` and `auto-maintain` are created automatically on first
   run; rename in the prompts if you prefer.

## 4. How the pieces fit

```
docs/ push ─▶ docs-watch ─▶ (workflow_dispatch) ─▶ generate-backlog
                                                     ├─ detect  (sha256 vs specs/backlog-state.json)
                                                     ├─ backlog (Claude files issues for changed docs)
                                                     └─ persist (commits updated state)
issue labelled auto-maintain ─▶ auto-maintain:
    implement (runner bases auto/issue-N on latest main → Claude implements, commits, pushes, OPENS the PR)
    └─ resolve PR ─▶ comment session cost (reusable-pr-comment)
    the PR triggers ci.yml (fmt · build · clippy · test) ─▶ human merge
```

Split of responsibility: the runner deterministically **bases the branch on the
latest `main`** (so the session never spends turns on that), and Claude does the
implementation **and opens the PR itself**. The PR must be opened by Claude's app
token — the Actions `GITHUB_TOKEN` may not create PRs, and PRs it did create
would not trigger `ci.yml`. Correctness is validated by CI **on the PR**, not in
the paid session, so a session is never wasted failing checks afterwards.

- **Memory:** `specs/backlog-state.json` holds a `sha256` per doc. Only new/
  changed docs are processed; delete an entry (or pass `-f force=true`) to
  reprocess.
- **Reusable + callers:** callers pass credentials with `secrets: inherit`, so
  no secret is ever written into a workflow file.
- **Cost reporting:** `reusable-claude.yml` reads the Claude execution log and
  exposes `cost_usd` + `num_turns` as outputs; `auto-maintain` resolves the PR
  for `auto/issue-N` and calls `reusable-pr-comment.yml` to post the session
  cost as a PR comment. Reuse the same block to report cost anywhere else.

## 5. Gotchas already solved (keep these!)

These are non-obvious and cost real debugging — don't "simplify" them away:

| Symptom | Fix (already in place) |
|---------|------------------------|
| `startup_failure` on every reusable call | **No `permissions:` block in `reusable-claude.yml`** — a reusable job requesting more than the caller grants aborts at startup. Each caller grants its own least-privilege set. |
| `Could not fetch an OIDC token` | **`id-token: write`** on every caller (OAuth uses OIDC). |
| `Unsupported event type: push` | **`docs-watch.yml`** translates a `push` into a `workflow_dispatch`; the Claude action rejects raw `push`. |
| `Workflow initiated by non-human actor` | **`allowed_bots: "*"`** in `reusable-claude.yml` (the watcher dispatches as the github-actions bot). |
| 8 permission denials, 0 issues created | Use broad **`Bash(gh:*)`**, not granular `Bash(gh issue create:*)` (multi-word prefixes don't match). |
| `GitHub Actions is not permitted to create or approve pull requests` | The Actions `GITHUB_TOKEN` cannot open PRs (org/repo setting), and PRs it opens would not trigger `ci.yml` anyway. **Let Claude open the PR** (its app token can, and it triggers CI). The runner only bases the branch on `main`. |
| Bot PR gets no CI run | Same root cause — a PR opened by `GITHUB_TOKEN` does not fire `on: pull_request`. Claude opening it (app token) does. Don't move PR creation into a runner step. |

## 6. First run

1. Push the files to `main`, install the Claude App, add the secrets.
2. Actions → **Generate Backlog from Docs** → Run workflow (tick *dry run* to
   preview). Issues appear labelled `spec` + `auto-maintain`.
3. Add the `auto-maintain` label to an issue (generated issues have it) → a PR
   opens on `auto/issue-<n>`. Review, let CI pass, merge.

Nothing merges automatically — a human always holds the merge button.
