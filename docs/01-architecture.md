# 01 · Architecture (the Rust harness)

The crate is a **functional-core agent harness**. The core is pure and
deterministic; effects live at the edges.

## Module map

| Module | Responsibility | Purity |
|--------|----------------|--------|
| `message` | `Role`, `Message`, immutable `Conversation` | pure |
| `tool` | `Tool` (a pure `fn(&str) -> Result<String, ToolError>`), `Toolbox` registry | pure |
| `agent` | `Agent` config + `Planner` (`fn(&Conversation) -> Decision`) | pure |
| `harness` | the fold loop: `run(agent, tools, task, max_steps) -> Run` | pure |
| `main` | wires a demo agent + tools and prints a transcript | edge (I/O) |

## Data flow

```
task ─▶ Conversation(system, user)
          │
          ▼   (fold, bounded by max_steps)
     Planner ── Decision::UseTool ─▶ Toolbox.invoke ─▶ observation ─▶ Conversation'
          │
          └──── Decision::Reply ────▶ Outcome::Replied(answer)  (stop)
```

## Design rules (enforced in review)

1. **Immutability.** `Conversation` and `Toolbox` never mutate in place;
   `with(...)` returns a new value.
2. **The planner is the only LLM seam.** Today it's a `fn` pointer. A real
   provider must slot in *there*, behind the same signature, without the core
   learning about the network.
3. **Errors are values.** Tools return `Result`; the loop turns a tool error
   into an observation rather than panicking.
4. **The loop is bounded.** Every run has a `max_steps` budget and reports
   `Outcome::Exhausted` when it is hit.

## Non-goals for the core

- No global mutable state, no `unsafe`, no panics on the happy or error path.
- No direct network or filesystem access inside `agent`/`harness`/`tool`/`message`.
