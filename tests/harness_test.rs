//! Behavioural tests for the functional harness core.

use code_from_docs::agent::{Agent, Call, Decision};
use code_from_docs::message::{Conversation, Message, Role};
use code_from_docs::tool::{Tool, ToolError, Toolbox};
use code_from_docs::{harness, Outcome};

fn reverse(input: &str) -> Result<String, ToolError> {
    Ok(input.chars().rev().collect())
}

/// Use `reverse` once, then reply with the observation.
fn tool_then_reply(conversation: &Conversation) -> Decision {
    match conversation.last() {
        Some(m) if m.role == Role::Tool => Decision::Reply(m.content.clone()),
        _ => Decision::UseTool {
            tool: "reverse".into(),
            input: "abc".into(),
        },
    }
}

fn always_reply(_conversation: &Conversation) -> Decision {
    Decision::Reply("hi".into())
}

fn never_reply(_conversation: &Conversation) -> Decision {
    Decision::UseTool {
        tool: "missing".into(),
        input: "x".into(),
    }
}

#[test]
fn tool_result_flows_into_reply() {
    let tools = Toolbox::new().with(Tool::new("reverse", "", reverse));
    let agent = Agent::new("t", "sys", tool_then_reply);
    let run = harness::run(&agent, &tools, "go", 5);
    assert_eq!(run.outcome, Outcome::Replied("cba".into()));
    assert_eq!(run.steps, 2);
}

#[test]
fn immediate_reply_takes_one_step() {
    let tools = Toolbox::new();
    let agent = Agent::new("t", "sys", always_reply);
    let run = harness::run(&agent, &tools, "go", 5);
    assert_eq!(run.outcome, Outcome::Replied("hi".into()));
    assert_eq!(run.steps, 1);
}

#[test]
fn loop_is_bounded_by_max_steps() {
    // `never_reply` calls a missing tool forever; the loop must stop at the budget.
    let tools = Toolbox::new();
    let agent = Agent::new("t", "sys", never_reply);
    let run = harness::run(&agent, &tools, "go", 3);
    assert_eq!(run.outcome, Outcome::Exhausted);
    assert_eq!(run.steps, 3);
}

#[test]
fn conversation_is_immutable() {
    let base = Conversation::new().with(Message::user("a"));
    let extended = base.with(Message::user("b"));
    assert_eq!(base.len(), 1);
    assert_eq!(extended.len(), 2);
}

#[test]
fn missing_tool_yields_not_found() {
    let tools = Toolbox::new();
    assert!(matches!(
        tools.invoke("nope", "x"),
        Err(ToolError::NotFound(_))
    ));
}

#[test]
fn count_role_empty_conversation() {
    let conv = Conversation::new();
    assert_eq!(conv.count_role(Role::User), 0);
    assert_eq!(conv.count_role(Role::Assistant), 0);
    assert_eq!(conv.count_role(Role::System), 0);
    assert_eq!(conv.count_role(Role::Tool), 0);
}

#[test]
fn count_role_user() {
    let conv = Conversation::new()
        .with(Message::user("a"))
        .with(Message::assistant("b"))
        .with(Message::user("c"));
    assert_eq!(conv.count_role(Role::User), 2);
}

#[test]
fn count_role_assistant() {
    let conv = Conversation::new()
        .with(Message::assistant("x"))
        .with(Message::user("y"))
        .with(Message::assistant("z"));
    assert_eq!(conv.count_role(Role::Assistant), 2);
}

#[test]
fn count_role_system() {
    let conv = Conversation::new()
        .with(Message::system("prompt"))
        .with(Message::user("hi"));
    assert_eq!(conv.count_role(Role::System), 1);
}

#[test]
fn count_role_tool() {
    let conv = Conversation::new()
        .with(Message::tool("result1"))
        .with(Message::tool("result2"))
        .with(Message::user("done"));
    assert_eq!(conv.count_role(Role::Tool), 2);
}

/// Planner that issues a two-tool batch on the first turn (the first call
/// intentionally targets an unregistered tool so it errors), then replies
/// with both observations joined by `|`.
fn two_tool_batch(conversation: &Conversation) -> Decision {
    let tool_msgs: Vec<_> = conversation
        .messages()
        .iter()
        .filter(|m| m.role == Role::Tool)
        .collect();

    if tool_msgs.is_empty() {
        Decision::UseTools(vec![
            Call {
                tool: "unknown".into(),
                input: "x".into(),
            },
            Call {
                tool: "reverse".into(),
                input: "abc".into(),
            },
        ])
    } else {
        let joined = tool_msgs
            .iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join("|");
        Decision::Reply(joined)
    }
}

#[test]
fn multi_tool_batch_first_call_errors_second_still_runs() {
    // "unknown" is not in the toolbox — produces a NotFound error observation.
    // "reverse" succeeds. Both observations must be appended before the next turn.
    let tools = Toolbox::new().with(Tool::new("reverse", "", reverse));
    let agent = Agent::new("t", "sys", two_tool_batch);
    let run = harness::run(&agent, &tools, "go", 5);

    assert_eq!(
        run.outcome,
        Outcome::Replied("error: no such tool: unknown|cba".into())
    );
    assert_eq!(run.steps, 2);

    // Both tool observations must appear in the transcript in call order.
    let tool_contents: Vec<_> = run
        .conversation
        .messages()
        .iter()
        .filter(|m| m.role == Role::Tool)
        .map(|m| m.content.as_str())
        .collect();
    assert_eq!(tool_contents, ["error: no such tool: unknown", "cba"]);
}
