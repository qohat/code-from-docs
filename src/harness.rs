//! The run loop: a pure state machine that folds decisions until the agent
//! replies or a step budget is exhausted.

use crate::agent::{Agent, Decision};
use crate::message::{Conversation, Message};
use crate::tool::Toolbox;

/// How a [`Run`] ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The agent produced a final answer.
    Replied(String),
    /// The loop hit `max_steps` without a reply.
    Exhausted,
}

/// The full result of a run: the transcript, the outcome and the step count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub conversation: Conversation,
    pub outcome: Outcome,
    pub steps: usize,
}

/// Advance the conversation by exactly one decision.
///
/// Pure: returns the next conversation plus `Some(answer)` when the agent chose
/// to reply (signalling the loop to stop) or `None` when it used a tool.
fn step(
    agent: &Agent,
    tools: &Toolbox,
    conversation: Conversation,
) -> (Conversation, Option<String>) {
    match agent.decide(&conversation) {
        Decision::Reply(answer) => {
            let next = conversation.with(Message::assistant(answer.clone()));
            (next, Some(answer))
        }
        Decision::UseTool { tool, input } => {
            let observation = match tools.invoke(&tool, &input) {
                Ok(out) => out,
                Err(err) => format!("error: {err}"),
            };
            let next = conversation
                .with(Message::assistant(format!("call {tool}({input})")))
                .with(Message::tool(observation));
            (next, None)
        }
    }
}

/// Run the agent to completion, or until `max_steps`.
///
/// Seeds the conversation with the system prompt and the user's task, then
/// folds [`step`] until the agent replies.
pub fn run(
    agent: &Agent,
    tools: &Toolbox,
    task: impl Into<String>,
    max_steps: usize,
) -> Run {
    let mut conversation = Conversation::new()
        .with(Message::system(agent.system_prompt.clone()))
        .with(Message::user(task));

    for steps in 1..=max_steps {
        let (next, reply) = step(agent, tools, conversation);
        conversation = next;
        if let Some(answer) = reply {
            return Run { conversation, outcome: Outcome::Replied(answer), steps };
        }
    }

    Run { conversation, outcome: Outcome::Exhausted, steps: max_steps }
}
