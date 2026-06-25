use crate::protocol::message::ChatMessage;
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

// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::message::Role;

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
}
