# Docs — source of truth

These documents describe **what the app should do**. They are the single input
that `generate-backlog.yml` reads to produce the GitHub issue backlog.

Rule of thumb: **if it isn't in `docs/`, it isn't a spec.** Change behaviour by
editing docs first; the automation turns the delta into issues, and
`auto-maintain.yml` turns issues into pull requests.

| File | What it covers |
|------|----------------|
| [00-vision.md](00-vision.md) | The product, who it's for, the loop it automates |
| [01-architecture.md](01-architecture.md) | The Rust harness: functional core, module map |
| [02-agent-harness.md](02-agent-harness.md) | Detailed capability specs (implemented + planned) |
| [03-roadmap.md](03-roadmap.md) | Phased roadmap and non-goals |

Each capability in `02-agent-harness.md` is tagged:

- ✅ **Implemented** — already in `src/`, do not re-file.
- 🚧 **Planned** — not yet built; the backlog generator should file an issue.
