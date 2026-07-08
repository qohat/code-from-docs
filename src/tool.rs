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

/// Configurable retry policy for transient tool failures.
///
/// `max_attempts` is the total number of invocations (including the first).
/// `should_retry` decides whether a given error warrants another attempt.
/// No sleeping occurs in the core; the edge layer may use [`RetryPolicy::backoff_ms`]
/// to compute the delay before actually retrying.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub should_retry: fn(&ToolError) -> bool,
}

impl RetryPolicy {
    /// Pure exponential back-off schedule. Returns milliseconds to wait before
    /// the attempt at `attempt` (0-indexed). Capped at attempt index 6 (~6.4 s).
    /// No sleeping happens in the core — the edge layer consumes this value.
    pub fn backoff_ms(attempt: usize) -> u64 {
        100u64 * (1u64 << attempt.min(6))
    }
}

/// A view of a [`Tool`] with a retry policy applied.
///
/// Created by [`Tool::with_retry`]. Calling [`invoke`](RetryWrapper::invoke)
/// drives up to `policy.max_attempts` attempts, recording each attempt as a
/// separate observation so the harness can append them to the transcript.
pub struct RetryWrapper {
    inner: Tool,
    policy: RetryPolicy,
}

impl RetryWrapper {
    /// Invoke the inner tool, retrying according to the policy.
    ///
    /// Returns `(observations, result)`:
    /// - `observations`: one entry per attempt — error messages for failed
    ///   attempts, the raw output string for the successful attempt.
    /// - `result`: the final `Ok` after a success, or the last `Err`.
    pub fn invoke(&self, input: &str) -> (Vec<String>, Result<String, ToolError>) {
        let mut observations = Vec::new();
        let mut last_err = ToolError::InvalidInput("no attempts made".into());

        for attempt in 0..self.policy.max_attempts {
            match self.inner.invoke(input) {
                Ok(out) => {
                    observations.push(out.clone());
                    return (observations, Ok(out));
                }
                Err(err) => {
                    observations.push(format!("attempt {} error: {err}", attempt + 1));
                    if !(self.policy.should_retry)(&err) {
                        return (observations, Err(err));
                    }
                    last_err = err;
                }
            }
        }

        (observations, Err(last_err))
    }
}

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
        Self {
            name: name.into(),
            description: description.into(),
            run,
        }
    }

    pub fn invoke(&self, input: &str) -> Result<String, ToolError> {
        (self.run)(input)
    }

    /// Wrap this tool with a retry policy.
    ///
    /// The returned [`RetryWrapper`] records each attempt as a distinct
    /// observation string, so the harness can append them to the transcript
    /// one by one.
    pub fn with_retry(&self, policy: RetryPolicy) -> RetryWrapper {
        RetryWrapper {
            inner: self.clone(),
            policy,
        }
    }
}

/// An immutable registry of tools keyed by name.
#[derive(Debug, Clone, Default)]
pub struct Toolbox {
    tools: BTreeMap<String, Tool>,
    policies: BTreeMap<String, RetryPolicy>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
            policies: BTreeMap::new(),
        }
    }

    /// Returns a new toolbox with `tool` added (replacing any of the same name).
    #[must_use]
    pub fn with(&self, tool: Tool) -> Self {
        let mut tools = self.tools.clone();
        tools.insert(tool.name.clone(), tool);
        Self {
            tools,
            policies: self.policies.clone(),
        }
    }

    /// Returns a new toolbox with `policy` associated with the named tool.
    #[must_use]
    pub fn with_policy(&self, name: impl Into<String>, policy: RetryPolicy) -> Self {
        let mut policies = self.policies.clone();
        policies.insert(name.into(), policy);
        Self {
            tools: self.tools.clone(),
            policies,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }

    pub fn invoke(&self, name: &str, input: &str) -> Result<String, ToolError> {
        self.get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?
            .invoke(input)
    }

    /// Invoke a tool, applying its retry policy if one is set.
    ///
    /// Returns one observation string per attempt. When no policy is registered
    /// for `name` this is always a single-element vec.
    pub fn invoke_observations(&self, name: &str, input: &str) -> Vec<String> {
        let tool = match self.get(name) {
            Some(t) => t,
            None => {
                return vec![format!("error: {}", ToolError::NotFound(name.to_string()))];
            }
        };
        if let Some(policy) = self.policies.get(name) {
            let (obs, _result) = tool.with_retry(*policy).invoke(input);
            obs
        } else {
            let single = match tool.invoke(input) {
                Ok(out) => out,
                Err(err) => format!("error: {err}"),
            };
            vec![single]
        }
    }

    /// Tool names in sorted order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.tools.keys().map(String::as_str)
    }
}
