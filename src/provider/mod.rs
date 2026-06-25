use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

pub mod mock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl ChatRequest {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            temperature: None,
            max_tokens: None,
        }
    }
}

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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatStreamChunk {
    pub delta: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

pub type ChatStream = Pin<Box<dyn Stream<Item = anyhow::Result<ChatStreamChunk>> + Send>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete(&self, request: ChatRequest) -> anyhow::Result<ChatResponse>;
}

#[async_trait]
pub trait Streamable: LlmProvider {
    /// Submit a [`ChatRequest`] and receive a [`ChatStream`] of chunks.
    async fn stream(&self, request: ChatRequest) -> anyhow::Result<ChatStream>;
}

// TESTS
#[cfg(test)]
mod tests;
