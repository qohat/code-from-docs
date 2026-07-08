//! Snapshot tests for the transcript renderers.

use code_from_docs::message::{Conversation, Message};
use code_from_docs::render::{render_compact, render_markdown};

fn sample_conversation() -> Conversation {
    Conversation::new()
        .with(Message::system("You are a helpful assistant."))
        .with(Message::user("Reverse 'abc'."))
        .with(Message::tool("cba"))
        .with(Message::assistant("The reversed string is `cba`."))
}

#[test]
fn render_markdown_snapshot() {
    let conv = sample_conversation();
    let md = render_markdown(&conv);
    let expected = "\
## System

You are a helpful assistant.

## User

Reverse 'abc'.

## Tool

cba

## Assistant

The reversed string is `cba`.
";
    assert_eq!(md, expected);
}

#[test]
fn render_compact_snapshot() {
    let conv = sample_conversation();
    let compact = render_compact(&conv);
    let expected = "\
system: You are a helpful assistant.
user: Reverse 'abc'.
tool: cba
assistant: The reversed string is `cba`.
";
    assert_eq!(compact, expected);
}

#[test]
fn render_markdown_empty_conversation() {
    let conv = Conversation::new();
    assert_eq!(render_markdown(&conv), "");
}

#[test]
fn render_compact_empty_conversation() {
    let conv = Conversation::new();
    assert_eq!(render_compact(&conv), "");
}

#[test]
fn renderers_are_pure_no_mutation() {
    let conv = sample_conversation();
    let _md1 = render_markdown(&conv);
    let _md2 = render_markdown(&conv);
    let _c1 = render_compact(&conv);
    let _c2 = render_compact(&conv);
    // Original conversation unchanged
    assert_eq!(conv.len(), 4);
}
