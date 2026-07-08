//! Conversation primitives — pure, immutable value types.

/// Who authored a [`Message`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// A single turn in a conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Self::new(Role::Tool, content)
    }
}

/// An immutable conversation transcript.
///
/// Every mutating-looking method returns a *new* value and leaves the receiver
/// untouched, so a conversation can be shared and folded over freely.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Returns a new conversation with `message` appended.
    #[must_use]
    pub fn with(&self, message: Message) -> Self {
        let mut messages = self.messages.clone();
        messages.push(message);
        Self { messages }
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn last(&self) -> Option<&Message> {
        self.messages.last()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl FromIterator<Message> for Conversation {
    fn from_iter<I: IntoIterator<Item = Message>>(iter: I) -> Self {
        Self {
            messages: iter.into_iter().collect(),
        }
    }
}
