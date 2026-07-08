//! Tools are pure functions from a string input to a string output.
//!
//! Keeping tools pure (a plain `fn` pointer, not a closure over mutable state)
//! means the whole harness stays deterministic and trivially testable.

use std::collections::BTreeMap;
use std::fmt;

/// Why a tool invocation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    NotFound(String),
    InvalidInput(String),
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolError::NotFound(name) => write!(f, "no such tool: {name}"),
            ToolError::InvalidInput(why) => write!(f, "invalid input: {why}"),
        }
    }
}

impl std::error::Error for ToolError {}

/// A named, side-effect-free capability the agent can invoke.
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    run: fn(&str) -> Result<String, ToolError>,
}

impl Tool {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        run: fn(&str) -> Result<String, ToolError>,
    ) -> Self {
        Self { name: name.into(), description: description.into(), run }
    }

    pub fn invoke(&self, input: &str) -> Result<String, ToolError> {
        (self.run)(input)
    }
}

/// An immutable registry of tools keyed by name.
#[derive(Debug, Clone, Default)]
pub struct Toolbox {
    tools: BTreeMap<String, Tool>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self { tools: BTreeMap::new() }
    }

    /// Returns a new toolbox with `tool` added (replacing any of the same name).
    #[must_use]
    pub fn with(&self, tool: Tool) -> Self {
        let mut tools = self.tools.clone();
        tools.insert(tool.name.clone(), tool);
        Self { tools }
    }

    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }

    pub fn invoke(&self, name: &str, input: &str) -> Result<String, ToolError> {
        self.get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?
            .invoke(input)
    }

    /// Tool names in sorted order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.tools.keys().map(String::as_str)
    }
}
