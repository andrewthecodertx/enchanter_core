use crate::provider::{
    ChatMessage, ChatRequest, ChatResponse, ChatStream, ChatStreamChunk, LlmProvider, Streamable,
    Usage,
};
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

// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ChatMessage, ChatRequest, Role};
    use futures::StreamExt;

    fn basic_request() -> ChatRequest {
        ChatRequest::new("test-model", vec![ChatMessage::user("Hello!")])
    }

    #[tokio::test]
    async fn test_mock_provider_returns_response() {
        let provider = MockProvider::new("Hello from mock!");
        let response = provider.complete(basic_request()).await.unwrap();

        assert_eq!(response.message.role, Role::Assistant);
        assert_eq!(response.message.content, "Hello from mock!");
        assert_eq!(response.finish_reason.as_deref(), Some("stop"));
    }

    #[tokio::test]
    async fn test_mock_provider_reports_usage() {
        let provider = MockProvider::new("Hi!");
        let response = provider.complete(basic_request()).await.unwrap();
        let usage = response.usage.expect("usage should be present");

        assert_eq!(
            usage.total_tokens,
            usage.prompt_tokens + usage.completion_tokens
        );
    }

    #[tokio::test]
    async fn test_provider_via_trait_object() {
        let provider: Box<dyn LlmProvider> = Box::new(MockProvider::new("boxed!"));
        let response = provider.complete(basic_request()).await.unwrap();

        assert_eq!(response.message.content, "boxed!");
    }

    #[tokio::test]
    async fn test_stream_yields_chunks() {
        let provider = MockStreamProvider::new("one two three");
        let mut stream = provider.stream(basic_request()).await.unwrap();

        let mut collected = String::new();
        while let Some(chunk) = stream.next().await {
            collected.push_str(&chunk.unwrap().delta);
        }

        assert_eq!(collected.trim(), "one two three");
    }

    #[tokio::test]
    async fn test_stream_provider_also_completes() {
        let provider = MockStreamProvider::new("works both ways");
        let response = provider.complete(basic_request()).await.unwrap();

        assert_eq!(response.message.content, "works both ways");
    }
}
