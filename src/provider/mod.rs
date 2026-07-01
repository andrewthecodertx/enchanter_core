use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::protocol::{ChatRequest, ChatResponse, ChatStreamChunk};

pub use openai::{Provider, OpenAiProvider};
pub mod mock;
pub mod openai;

pub type ChatStream = Pin<Box<dyn Stream<Item = anyhow::Result<ChatStreamChunk>> + Send>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete(&self, request: ChatRequest) -> anyhow::Result<ChatResponse>;
}

#[async_trait]
pub trait Streamable: LlmProvider {
    async fn stream(&self, request: ChatRequest) -> anyhow::Result<ChatStream>;
}

