use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("unknown tool: '{0}'")]
    NotFound(String),

    #[error("invalid arguments for tool '{tool}': {reason}")]
    InvalidArguments { tool: String, reason: String },

    #[error("tool '{tool}' failed: {reason}")]
    ExecutionFailed { tool: String, reason: String },

    #[error("tool '{0}' timed out")]
    TimedOut(String),
}

impl ToolError {
    pub fn tool_name(&self) -> Option<&str> {
        match self {
            ToolError::NotFound(name) => Some(name),
            ToolError::InvalidArguments { tool, .. } => Some(tool),
            ToolError::ExecutionFailed { tool, .. } => Some(tool),
            ToolError::TimedOut(name) => Some(name),
        }
    }

    pub fn is_recoverable(&self) -> bool {
        match self {
            ToolError::NotFound(_) => false,
            ToolError::InvalidArguments { .. } => true,
            ToolError::ExecutionFailed { .. } => true,
            ToolError::TimedOut(_) => true,
        }
    }
}
