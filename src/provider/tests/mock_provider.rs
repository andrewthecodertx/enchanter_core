use crate::provider::mock::{MockProvider, MockStreamProvider};
use crate::provider::{ChatMessage, ChatRequest, LlmProvider, Role, Streamable};
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
