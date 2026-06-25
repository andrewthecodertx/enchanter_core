use crate::protocol::message::ChatMessage;
use crate::protocol::response::{ChatResponse, ChatStreamChunk, Usage};

fn make_response(finish_reason: &str) -> ChatResponse {
    ChatResponse {
        message: ChatMessage::assistant("Hello!"),
        finish_reason: Some(finish_reason.to_string()),
        usage: None,
    }
}

#[test]
fn is_complete_stop() {
    assert!(make_response("stop").is_complete());
    assert!(!make_response("length").is_complete());
}

#[test]
fn is_tool_call() {
    assert!(make_response("tool_calls").is_tool_call());
    assert!(!make_response("stop").is_tool_call());
}

#[test]
fn content_accessor() {
    let r = make_response("stop");
    assert_eq!(r.content(), "Hello!");
}

#[test]
fn usage_optional_in_serialization() {
    let r = make_response("stop");
    let json = serde_json::to_string(&r).unwrap();
    assert!(!json.contains("usage"));
}

#[test]
fn stream_chunk_is_final() {
    let chunk = ChatStreamChunk {
        delta: "".to_string(),
        finish_reason: Some("stop".to_string()),
    };
    assert!(chunk.is_final());

    let chunk = ChatStreamChunk {
        delta: "Hello".to_string(),
        finish_reason: None,
    };
    assert!(!chunk.is_final());
}

#[test]
fn roundtrip_response() {
    let original = ChatResponse {
        message: ChatMessage::assistant("Hi!"),
        finish_reason: Some("stop".to_string()),
        usage: Some(Usage {
            prompt_tokens: 10,
            completion_tokens: 5,
            total_tokens: 15,
        }),
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: ChatResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}
