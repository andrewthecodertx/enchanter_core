use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: String) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn system(content: String) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub messages: ChatMessage,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize)]
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
    async fn stream(&self, request: ChatRequest) -> anyhow::Result<ChatStream>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockProvider {
        response: String,
    }

    impl MockProvider {
        fn new(response: impl Into<String>) -> Self {
            Self {
                response: response.into(),
            }
        }
    }

    #[async_trait]
    impl LlmProvider for MockProvider {
        async fn complete(&self, _request: ChatRequest) -> anyhow::Result<ChatResponse> {
            Ok(ChatResponse {
                messages: ChatMessage::assistant(self.response.clone()),
                finish_reason: Some("stop".into()),
                usage: Some(Usage {
                    prompt_tokens: 10,
                    completion_tokens: 5,
                    total_tokens: 15,
                }),
            })
        }
    }

    struct MockStreamProvider {
        response: String,
    }

    impl MockStreamProvider {
        fn new(response: impl Into<String>) -> Self {
            Self {
                response: response.into(),
            }
        }
    }

    #[async_trait]
    impl LlmProvider for MockStreamProvider {
        async fn complete(&self, _request: ChatRequest) -> anyhow::Result<ChatResponse> {
            Ok(ChatResponse {
                messages: ChatMessage::assistant(self.response.clone()),
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

    fn basic_request() -> ChatRequest {
        ChatRequest::new(
            "test-model",
            vec![ChatMessage::user("Hello, how are you?".into())],
        )
    }

    #[test]
    fn test_message_constructors() {
        let user = ChatMessage::user("Hello, how are you?".into());
        let assistant = ChatMessage::assistant("I'm doing well, thank you for asking.".into());
        let system = ChatMessage::system("This is a test system message.".into());

        assert_eq!(user.role, Role::User);
        assert_eq!(assistant.role, Role::Assistant);
        assert_eq!(system.role, Role::System);
    }

    #[test]
    fn test_chat_request_new() {
        let request = basic_request();

        assert_eq!(request.model, "test-model");
        assert_eq!(request.messages.len(), 1);

        assert!(request.temperature.is_none());
        assert!(request.max_tokens.is_none());
    }

    // LLM provider tests
    #[tokio::test]
    async fn test_mock_provider_reports_usage() {
        let provider = MockProvider::new("Hello, how are you?");
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

        assert_eq!(response.messages.content, "boxed!");
    }

    // Streamable provider tests
    #[tokio::test]
    async fn test_stream_yields_chunks() {
        use futures::StreamExt;

        let provider = MockStreamProvider::new("Hello, how are you?");
        let mut stream = provider.stream(basic_request()).await.unwrap();

        let mut collected_chunks = String::new();
        while let Some(chunk) = stream.next().await {
            collected_chunks.push_str(&chunk.unwrap().delta);
        }

        assert_eq!(collected_chunks.trim(), "Hello, how are you?");
    }

    #[tokio::test]
    async fn test_stream_completes() {
        let provider = MockStreamProvider::new("works both ways");
        let response = provider.complete(basic_request()).await.unwrap();

        assert_eq!(response.messages.content, "works both ways");
    }

    #[test]
    fn test_role_serialization_lowercase() {
        assert_eq!(serde_json::to_string(&Role::User).unwrap(), "\"user\"");
        assert_eq!(
            serde_json::to_string(&Role::Assistant).unwrap(),
            "\"assistant\""
        );
        assert_eq!(serde_json::to_string(&Role::System).unwrap(), "\"system\"");
    }

    #[test]
    fn test_chat_request_roundtrip() {
        let request = basic_request();
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ChatRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.messages.len(), request.messages.len());
        assert_eq!(deserialized.model, request.model);
    }

    #[test]
    fn test_optional_fields_ommitted() {
        let request = basic_request();
        let serialized = serde_json::to_string(&request).unwrap();

        assert!(!serialized.contains("temperature"));
        assert!(!serialized.contains("max_tokens"));
    }
}
