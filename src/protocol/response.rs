use crate::protocol::message::ChatMessage;
use crate::protocol::tools::ToolCall;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

impl ChatResponse {
    pub fn is_tool_call(&self) -> bool {
        self.finish_reason.as_deref() == Some("tool_calls")
    }

    pub fn is_complete(&self) -> bool {
        self.finish_reason.as_deref() == Some("stop")
    }

    pub fn content(&self) -> &str {
        &self.message.content
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamChunk {
    pub delta: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

impl ChatStreamChunk {
    pub fn is_final(&self) -> bool {
        self.finish_reason.is_some()
    }
}
