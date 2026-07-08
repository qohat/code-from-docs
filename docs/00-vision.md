# 00 · Vision

## What this is

**code-from-docs** is a demo of a *doc-driven code generation pipeline*. The
product is the pipeline itself; the Rust crate in `src/` is the small,
real codebase the pipeline operates on so the loop is observable end-to-end.

## The loop

```
        edit docs/                 generate-backlog.yml            auto-maintain.yml
 human ───────────────▶ docs ───────────────────────▶ GitHub issues ───────────────▶ pull requests ──▶ human review
                          ▲                (Claude reads the delta)     (Claude implements each issue)          │
                          └──────────────────────────── behaviour changes land as new docs ────────────────────┘
```

1. A human edits `docs/` — the source of truth for behaviour.
2. `generate-backlog.yml` runs Claude Code, which reads the docs and the current
   `src/`, and files one GitHub issue per **not-yet-implemented** capability.
3. `auto-maintain.yml` picks up each `auto-maintain`-labelled issue, implements
   it on a branch, and opens a pull request.
4. A human reviews and merges. Nothing merges automatically.

## Who it's for

Solo builders and small teams who want the backlog and first-draft
implementation to fall out of well-kept documentation instead of being
hand-transcribed into a tracker.

## Why a Rust agent-harness as the demo target

- Small enough to reason about in one sitting.
- A **functional core** (pure functions, immutable values) makes generated
  changes easy to review and hard to break subtly.
- "An agent harness that builds itself with an agent harness" is the point.

## Principles

- **Docs are the spec.** Code without a doc is drift; a doc without code is a
  backlog item.
- **Humans hold the merge button.** The automation proposes; it never merges.
- **Secrets never touch the repo.** All credentials live in GitHub secrets and
  are surfaced only through `.env.example` as names.
- **Small, independent issues.** Each issue should be reviewable on its own.
