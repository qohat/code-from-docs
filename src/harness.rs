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
            let obs_list = tools.invoke_observations(&tool, &input);
            let next = obs_list.iter().fold(
                conversation.with(Message::assistant(format!("call {tool}({input})"))),
                |conv, obs| conv.with(Message::tool(obs.clone())),
            );
            (next, None)
        }
        Decision::UseTools(calls) => {
            let label = calls
                .iter()
                .map(|c| format!("call {}({})", c.tool, c.input))
                .collect::<Vec<_>>()
                .join(", ");
            let with_label = conversation.with(Message::assistant(label));
            let next = calls.iter().fold(with_label, |conv, call| {
                let obs_list = tools.invoke_observations(&call.tool, &call.input);
                obs_list
                    .iter()
                    .fold(conv, |c, obs| c.with(Message::tool(obs.clone())))
            });
            (next, None)
        }
    }
}

/// Run the agent to completion, or until `max_steps`.
///
/// Seeds the conversation with the system prompt and the user's task, then
/// folds [`step`] until the agent replies.
pub fn run(agent: &Agent, tools: &Toolbox, task: impl Into<String>, max_steps: usize) -> Run {
    let mut conversation = Conversation::new()
        .with(Message::system(agent.system_prompt.clone()))
        .with(Message::user(task));

    for steps in 1..=max_steps {
        let (next, reply) = step(agent, tools, conversation);
        conversation = next;
        if let Some(answer) = reply {
            return Run {
                conversation,
                outcome: Outcome::Replied(answer),
                steps,
            };
        }
    }

    Run {
        conversation,
        outcome: Outcome::Exhausted,
        steps: max_steps,
    }
}
