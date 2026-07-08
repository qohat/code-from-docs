//! A runnable demo of the functional agent harness.
//!
//! Run with `cargo run` — it wires up two pure tools and a deterministic
//! planner, then prints the resulting transcript.

use code_from_docs::agent::{Agent, Decision};
use code_from_docs::message::{Conversation, Role};
use code_from_docs::tool::{Tool, ToolError, Toolbox};
use code_from_docs::{harness, Outcome};

/// Reverse the input string. Pure.
fn reverse(input: &str) -> Result<String, ToolError> {
    Ok(input.chars().rev().collect())
}

/// Uppercase the input string. Pure.
fn shout(input: &str) -> Result<String, ToolError> {
    if input.is_empty() {
        return Err(ToolError::InvalidInput("empty input".into()));
    }
    Ok(input.to_uppercase())
}

/// A deterministic stand-in for an LLM: once a tool has produced an
/// observation, reply with it; otherwise call `reverse` on the user's task.
fn planner(conversation: &Conversation) -> Decision {
    match conversation.last() {
        Some(m) if m.role == Role::Tool => Decision::Reply(format!("done: {}", m.content)),
        _ => {
            let task = conversation
                .messages()
                .iter()
                .find(|m| m.role == Role::User)
                .map(|m| m.content.clone())
                .unwrap_or_default();
            Decision::UseTool {
                tool: "reverse".into(),
                input: task,
            }
        }
    }
}

fn main() {
    let tools = Toolbox::new()
        .with(Tool::new("reverse", "Reverse the input string", reverse))
        .with(Tool::new("shout", "Uppercase the input string", shout));

    let agent = Agent::new(
        "demo",
        "You are a tiny demo agent. Use a tool, then answer.",
        planner,
    );

    let run = harness::run(&agent, &tools, "hello harness", 8);

    println!("agent : {}", agent.name);
    println!("tools : {}", tools.names().collect::<Vec<_>>().join(", "));
    println!("steps : {}", run.steps);
    match &run.outcome {
        Outcome::Replied(answer) => println!("reply : {answer}"),
        Outcome::Exhausted => println!("reply : <exhausted>"),
    }

    println!("\ntranscript:");
    for msg in run.conversation.messages() {
        println!("  [{:?}] {}", msg.role, msg.content);
    }
}
