# 02 · Agent harness — capability specs

Each capability is a unit of behaviour. The backlog generator files an issue for
every 🚧 **Planned** capability that has no matching open issue, and skips every
✅ **Implemented** one.

Keep specs small and independently shippable.

---

## Implemented

### ✅ C1 — Conversation transcript
Immutable `Conversation` of `Message`s with roles `System | User | Assistant |
Tool`. `with(msg)` returns a new conversation.

### ✅ C2 — Pure tools + registry
`Tool` wraps a pure `fn(&str) -> Result<String, ToolError>`. `Toolbox` is an
immutable registry with `invoke(name, input)`.

### ✅ C3 — Planner + Agent
`Agent` holds a system prompt and a `Planner` (`fn(&Conversation) -> Decision`)
returning `Reply` or `UseTool`.

### ✅ C4 — Bounded run loop
`run(agent, tools, task, max_steps)` folds decisions, feeds tool observations
back into the conversation, and stops on reply or `max_steps`.

---

## Planned

### ✅ C5 — Multi-tool decisions
Allow the planner to return **several** tool calls from one decision and have the
loop execute them (left to right), appending all observations before the next
planner turn.
**Acceptance:** `Decision::UseTools(Vec<Call>)`; observations preserve order; a
failing call becomes an error observation and does not abort the batch.

### ✅ C6 — Retry policy for tools
A configurable retry wrapper for flaky tools: retry up to N times on
`ToolError`, with a pure backoff schedule (no real sleeping in tests).
**Acceptance:** `RetryPolicy { max_attempts, should_retry: fn(&ToolError)->bool }`;
the harness records each attempt in the transcript; deterministic under tests.

### ✅ C7 — Structured tool I/O
Let a tool declare a typed input/output via `serde` so the planner can pass
JSON instead of a bare string, while keeping the pure-function contract.
**Acceptance:** a `TypedTool<In, Out>` adapter; existing string tools still work;
invalid JSON yields `ToolError::InvalidInput`.

### 🚧 C8 — Token / step budget accounting
Track a cost estimate per run (steps taken, tool calls, chars produced) and
return it in `Run`.
**Acceptance:** `Run.usage: Usage { steps, tool_calls, chars }`; unit-tested on
the demo scenario.

### ✅ C9 — Transcript rendering
A pure function that renders a `Conversation` to Markdown (and to a compact
one-line-per-turn form) for logs and PR descriptions.
**Acceptance:** `render_markdown(&Conversation) -> String`; snapshot-tested;
no I/O.

### 🚧 C10 — Provider trait (LLM seam)
Introduce a `Provider` trait behind the `Planner` seam so a real LLM can back the
planner, with the pure `fn` planner remaining the default/test double.
**Acceptance:** `trait Provider { fn plan(&self, &Conversation) -> Decision }`;
core modules stay free of network code; the demo still runs offline.

### ✅ C11 — Conversation role counting
A pure helper to count how many messages in a `Conversation` have a given role,
useful for inspection and budgeting.
**Acceptance:** `Conversation::count_role(&self, role: Role) -> usize`; does not
allocate a new conversation; unit-tested; no I/O.
