//! The agent's decision function and configuration.

use crate::message::Conversation;

/// What the agent decides to do after observing the conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Emit a final reply and stop the loop.
    Reply(String),
    /// Invoke a tool with the given input, then continue looping.
    UseTool { tool: String, input: String },
}

/// A `Planner` maps the current conversation to the next [`Decision`].
///
/// This is the single seam where a real LLM would plug in. Here it is an
/// ordinary `fn` pointer, which keeps the harness pure and deterministic.
pub type Planner = fn(&Conversation) -> Decision;

/// Immutable agent definition.
#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub system_prompt: String,
    pub planner: Planner,
}

impl Agent {
    pub fn new(
        name: impl Into<String>,
        system_prompt: impl Into<String>,
        planner: Planner,
    ) -> Self {
        Self { name: name.into(), system_prompt: system_prompt.into(), planner }
    }

    /// Ask the planner what to do next given `conversation`.
    pub fn decide(&self, conversation: &Conversation) -> Decision {
        (self.planner)(conversation)
    }
}
