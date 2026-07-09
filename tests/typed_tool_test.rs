//! Tests for TypedTool (C7 — structured tool I/O via serde).

use code_from_docs::agent::{Agent, Decision};
use code_from_docs::message::{Conversation, Role};
use code_from_docs::tool::{ToolError, Toolbox, TypedTool};
use code_from_docs::{harness, Outcome};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddInput {
    a: i64,
    b: i64,
}

#[derive(Serialize)]
struct AddOutput {
    sum: i64,
}

fn add(input: AddInput) -> Result<AddOutput, ToolError> {
    Ok(AddOutput {
        sum: input.a + input.b,
    })
}

#[test]
fn typed_tool_invoke_valid_json() {
    let tool: TypedTool<AddInput, AddOutput> = TypedTool::new("add", "sums two integers", add);
    let result = tool.invoke(r#"{"a":3,"b":4}"#).unwrap();
    assert_eq!(result, r#"{"sum":7}"#);
}

#[test]
fn typed_tool_invalid_json_returns_invalid_input_error() {
    let tool: TypedTool<AddInput, AddOutput> = TypedTool::new("add", "", add);
    let err = tool.invoke("not json at all").unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput(_)));
}

#[test]
fn typed_tool_missing_fields_returns_invalid_input_error() {
    let tool: TypedTool<AddInput, AddOutput> = TypedTool::new("add", "", add);
    // Missing required fields "a" and "b".
    let err = tool.invoke(r#"{"x":1}"#).unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput(_)));
}

fn use_add_tool(conv: &Conversation) -> Decision {
    let saw_tool = conv.messages().iter().any(|m| m.role == Role::Tool);
    if saw_tool {
        let result = conv
            .messages()
            .iter()
            .rfind(|m| m.role == Role::Tool)
            .map(|m| m.content.clone())
            .unwrap_or_default();
        Decision::Reply(result)
    } else {
        Decision::UseTool {
            tool: "add".into(),
            input: r#"{"a":10,"b":32}"#.into(),
        }
    }
}

#[test]
fn typed_tool_end_to_end_through_harness() {
    let tools = Toolbox::new().with(TypedTool::new("add", "sums two integers", add).into_tool());
    let agent = Agent::new("t", "sys", use_add_tool);
    let run = harness::run(&agent, &tools, "what is 10 + 32?", 5);
    assert_eq!(run.outcome, Outcome::Replied(r#"{"sum":42}"#.to_string()));
    assert_eq!(run.steps, 2);
}

#[test]
fn existing_string_tool_still_works_alongside_typed_tool() {
    use code_from_docs::tool::Tool;

    fn reverse(input: &str) -> Result<String, ToolError> {
        Ok(input.chars().rev().collect())
    }

    let tools = Toolbox::new()
        .with(Tool::new("reverse", "", reverse))
        .with(TypedTool::new("add", "sums two integers", add).into_tool());

    assert_eq!(tools.invoke("reverse", "abc").unwrap(), "cba");
    assert_eq!(
        tools.invoke("add", r#"{"a":1,"b":2}"#).unwrap(),
        r#"{"sum":3}"#
    );
}
