//! A tiny, functional-core agent harness.
//!
//! The design keeps the core **pure**: [`Conversation`], [`Tool`]s and the
//! [`Agent`]'s [`Planner`] are deterministic values / functions with no I/O.
//! Side effects (a real LLM call, real network access) belong only at the
//! edges — which is exactly where a real provider would later plug in.
//!
//! See `docs/` for the product-level description that drives the automation.

pub mod agent;
pub mod harness;
pub mod message;
pub mod tool;

pub use agent::{Agent, Call, Decision, Planner};
pub use harness::{run, Outcome, Run};
pub use message::{Conversation, Message, Role};
pub use tool::{Tool, ToolError, Toolbox};
