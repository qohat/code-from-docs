# Cost model — and why `GH_PAT` matters

`auto-maintain` pays for a Claude session per issue. This documents where that
money goes and why a fine-grained PAT (`GH_PAT`) makes runs both **cheaper** and
**guaranteed to pass CI**.

## Measured data

Same workflow, two designs, comparable issues (small, functional-core changes):

| Run | Design | Cost (USD) | Turns |
|-----|--------|-----------|-------|
| C6 → PR #11 | Claude did git **and** opened the PR in-session | **$1.33** | 39 |
| C9 → PR #12 | Runner does git/PR (`GH_PAT`); Claude only implements + validates | **$0.32** | 17 |

**≈ 76% cheaper, ≈ 57% fewer turns** — and the second design *guarantees* the PR
passes CI, which the first did not.

## Where the money went

Turn-by-turn analysis of the $1.33 run:

- `cargo build` / `clippy` / `test`: **0 runs** — the model never validated its
  own code, so a green CI was luck, not a guarantee.
- `gh pr create`: **~7 attempts** — the model repeatedly fumbled opening the PR
  (branch state, quoting, "PR already exists"). This retry loop, plus the
  surrounding `git commit`/`git push` attempts, was the real cost sink.

Conclusion: **git/PR plumbing is where sessions waste money**, not the checks.
Plumbing is deterministic work — it should not be done by a language model at all.

## Why we can't just use the free `GITHUB_TOKEN`

The obvious fix — "let the runner open the PR with the built-in Actions token" —
hits two hard GitHub limits:

1. `GITHUB_TOKEN` **cannot create pull requests** (`GitHub Actions is not
   permitted to create or approve pull requests`).
2. Even where it can push, **a PR opened by `GITHUB_TOKEN` does not trigger
   `on: pull_request`** (GitHub blocks this to prevent workflow loops). So
   `ci.yml` would never run on the bot's PRs.

So the token that opens the PR must be a **real-user token** — either the Claude
GitHub App token (only available *inside* the Claude session) or a PAT.

## The `GH_PAT` design

A fine-grained PAT (Contents + Pull requests: write) given to the **runner**
lets us move ALL git/PR work out of the paid session while still triggering CI:

```
runner:  branch from main → cargo fmt → commit → push → gh pr create   (GH_PAT, deterministic, $0 AI)
claude:  implement + `cargo build && clippy && test` until green        (paid — buys correctness only)
PR:      opened by GH_PAT → ci.yml runs (fmt · build · clippy · test)   → human merge
```

- **Cheaper:** the ~7-attempt `gh pr create` loop and all git plumbing leave the
  AI session entirely. The session pays only to write correct code.
- **Guaranteed green:** the prompt requires Claude to run the *exact* commands
  `ci.yml` runs and leave them passing. Green in-session ⇒ green on the PR.
- **CI actually runs:** because the PAT is a real user, the PR fires
  `on: pull_request`.

## Keeping cost down further

- **Small issues.** `generate-backlog` is told to file small, independent specs;
  a tight issue = a short session.
- **A tight prompt.** `.github/prompts/implement-issue.md` points Claude straight
  at the relevant files and gives one combined check command — less exploration.
- **No plumbing in the prompt.** Every git/`gh`/`fmt` instruction removed from the
  prompt is turns saved. The runner owns those.
- **Model choice.** `--model` in `auto-maintain.yml`'s `claude_args` trades cost
  for capability.

## Security note on the PAT

- Use a **fine-grained** PAT scoped to this repo only, with the minimum
  permissions (Contents + Pull requests: write).
- Store it as the `GH_PAT` Actions secret — never in the repo (`.env.example`
  documents the name only).
- Rotate it periodically; revoke immediately if leaked.
