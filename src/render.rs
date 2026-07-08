//! Pure transcript renderers — no I/O, no side effects.

use crate::message::{Conversation, Role};

impl Role {
    fn label(&self) -> &'static str {
        match self {
            Role::System => "System",
            Role::User => "User",
            Role::Assistant => "Assistant",
            Role::Tool => "Tool",
        }
    }

    fn compact_label(&self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::Tool => "tool",
        }
    }
}

/// Renders a [`Conversation`] as Markdown with a `##` heading per turn.
pub fn render_markdown(conversation: &Conversation) -> String {
    let messages = conversation.messages();
    if messages.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for (i, msg) in messages.iter().enumerate() {
        out.push_str("## ");
        out.push_str(msg.role.label());
        out.push_str("\n\n");
        out.push_str(&msg.content);
        out.push('\n');
        if i + 1 < messages.len() {
            out.push('\n');
        }
    }
    out
}

/// Renders a [`Conversation`] as one `role: content` line per turn.
pub fn render_compact(conversation: &Conversation) -> String {
    let mut out = String::new();
    for msg in conversation.messages() {
        out.push_str(msg.role.compact_label());
        out.push_str(": ");
        out.push_str(&msg.content);
        out.push('\n');
    }
    out
}
