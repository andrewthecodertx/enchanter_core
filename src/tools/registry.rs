use std::collections::HashMap;

use super::context::ToolContext;
use super::error::ToolError;
use super::tool::Tool;
use crate::protocol::message::ChatMessage;
use crate::protocol::tools::{Tool as ProtocolTool, ToolCall};

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: impl Tool + 'static) {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
    }

    pub fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn to_protocol_tools(&self) -> Vec<ProtocolTool> {
        self.tools.values().map(|t| t.to_protocol()).collect()
    }

    pub async fn execute(
        &self,
        call: &ToolCall,
        context: &ToolContext,
    ) -> Result<ChatMessage, ToolError> {
        let tool = self
            .tools
            .get(&call.function.name)
            .ok_or_else(|| ToolError::NotFound(call.function.name.clone()))?;

        let arguments: serde_json::Value =
            serde_json::from_str(&call.function.arguments).map_err(|e| {
                ToolError::InvalidArguments {
                    tool: call.function.name.clone(),
                    reason: e.to_string(),
                }
            })?;

        let result =
            tool.execute(arguments, context)
                .await
                .map_err(|e| ToolError::ExecutionFailed {
                    tool: call.function.name.clone(),
                    reason: e.to_string(),
                })?;

        Ok(ChatMessage::tool(&call.id, &result))
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
