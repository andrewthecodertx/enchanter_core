use crate::protocol::{ChatMessage, ChatRequest, ChatResponse, ChatStreamChunk, Usage};
use crate::provider::{ChatStream, LlmProvider, Streamable};
use async_trait::async_trait;

pub struct MockProvider {
    response: String,
}

impl MockProvider {
    pub fn new(response: impl Into<String>) -> Self {
        Self {
            response: response.into(),
        }
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    async fn complete(&self, _request: ChatRequest) -> anyhow::Result<ChatResponse> {
        Ok(ChatResponse {
            message: ChatMessage::assistant(&self.response),
            finish_reason: Some("stop".into()),
            usage: Some(Usage {
                prompt_tokens: 10,
                completion_tokens: 5,
                total_tokens: 15,
            }),
            tool_calls: Vec::new(),
        })
    }
}

pub struct MockStreamProvider {
    response: String,
}

impl MockStreamProvider {
    pub fn new(response: impl Into<String>) -> Self {
        Self {
            response: response.into(),
        }
    }
}

#[async_trait]
impl LlmProvider for MockStreamProvider {
    async fn complete(&self, _request: ChatRequest) -> anyhow::Result<ChatResponse> {
        Ok(ChatResponse {
            message: ChatMessage::assistant(&self.response),
            finish_reason: Some("stop".into()),
            usage: None,
            tool_calls: Vec::new(),
        })
    }
}

#[async_trait]
impl Streamable for MockStreamProvider {
    async fn stream(&self, _request: ChatRequest) -> anyhow::Result<ChatStream> {
        let words: Vec<String> = self
            .response
            .split_whitespace()
            .map(|w| format!("{} ", w))
            .collect();

        let stream = futures::stream::iter(words.into_iter().map(|word| {
            Ok(ChatStreamChunk {
                delta: word,
                finish_reason: None,
            })
        }));

        Ok(Box::pin(stream))
    }
}
