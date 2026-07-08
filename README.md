# code-from-docs

A demo of **doc-driven code generation** with GitHub Actions and Claude Code.

You keep the spec in `docs/`. One workflow turns the docs into a GitHub issue
backlog; another picks up each issue, writes the code on a branch, and opens a
pull request for you to review. The thing being built is a small **functional
agent harness** in Rust (`src/`) — small enough to watch the whole loop work.

```
edit docs/ ──▶ generate-backlog ──▶ GitHub issues ──▶ auto-maintain ──▶ pull requests ──▶ you review & merge
```

## Repository layout

```
.
├── docs/                     # SOURCE OF TRUTH — what the app should do
│   ├── 00-vision.md
│   ├── 01-architecture.md
│   ├── 02-agent-harness.md   # capability specs (Implemented / Planned)
│   └── 03-roadmap.md
├── src/                      # the Rust harness (the demo target)
│   ├── message.rs            # immutable Conversation
│   ├── tool.rs               # pure tools + registry
│   ├── agent.rs              # planner + agent
│   ├── harness.rs            # the bounded fold loop
│   ├── lib.rs
│   └── main.rs               # runnable demo
├── tests/                    # behavioural tests
├── specs/                    # (optional) persisted backlog artifacts
├── CLAUDE.md                 # conventions the coding agent must follow
├── .env.example              # required secret NAMES (no values)
└── .github/workflows/
    ├── generate-backlog.yml  # caller: docs  → issues
    ├── auto-maintain.yml     # caller: issue → PR
    ├── ci.yml                # quality gate: fmt · clippy · test on every PR
    ├── reusable-claude.yml   # reusable: runs claude-code-action headlessly
    └── reusable-discord.yml  # reusable: Discord notifications
```

## How the workflows fit together

The pipeline is split into **reusable** building blocks and thin **caller**
workflows that only orchestrate:

| Workflow | Trigger | Does |
|----------|---------|------|
| `generate-backlog.yml` | manual, or push to `docs/**` | reads docs + `src/`, files one issue per Planned capability (no duplicates) |
| `auto-maintain.yml` | issue labelled `auto-maintain`, or manual | implements the issue on `auto/issue-<n>` and opens a PR |
| `ci.yml` | every PR + push to `main` | `cargo fmt --check`, `clippy -D warnings`, `cargo test` — the gate for auto-generated PRs |
| `reusable-claude.yml` | `workflow_call` | checkout + `anthropics/claude-code-action@v1` (headless) |
| `reusable-discord.yml` | `workflow_call` | posts a success/failure embed to a Discord webhook |

Callers invoke the reusables with `uses:` and pass credentials with
`secrets: inherit`, so **no secret is ever written into a workflow file**.
Each caller declares least-privilege `permissions:` (backlog = `issues: write`
only; auto-maintain = `contents`/`issues`/`pull-requests: write`).

## Setup

1. **Push this repo to GitHub** and install the
   [Claude GitHub App](https://github.com/apps/claude) on it.
2. **Add repository secrets** (Settings → Secrets and variables → Actions).
   See [`.env.example`](.env.example) for the exact names:
   - `CLAUDE_CODE_OAUTH_TOKEN` — from `claude setup-token`.
   - `DISCORD_WEBHOOK_URL` — a Discord Incoming Webhook.
3. **Generate the backlog:** Actions → *Generate Backlog from Docs* → *Run
   workflow* (tick *dry run* first to preview). Issues appear labelled `spec`
   and `auto-maintain`.
4. **Let it code:** adding the `auto-maintain` label triggers
   *Auto-Maintain (Issue → PR)*, which opens a PR. Review and merge.

Nothing merges automatically — humans hold the merge button.

## Run the demo locally

```bash
cargo run            # prints a transcript from the demo agent
cargo test           # behavioural tests
cargo clippy --all-targets -- -D warnings
```

## Changing behaviour

Edit `docs/` first, then push. The delta becomes issues, issues become PRs.
`CLAUDE.md` defines the coding conventions the agent must follow, including
flipping a capability from 🚧 Planned to ✅ Implemented in the same PR so it
isn't re-filed.
