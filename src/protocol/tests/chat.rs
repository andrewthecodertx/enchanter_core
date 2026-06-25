use crate::protocol::chat::ChatRequest;
use crate::protocol::message::{ChatMessage, Role};

#[test]
fn builder_basic() {
    let req = ChatRequest::builder("llama3.2")
        .system("You are helpful.")
        .user("Hello!")
        .build();

    assert_eq!(req.model, "llama3.2");
    assert_eq!(req.messages.len(), 2);
    assert_eq!(req.messages[0].role, Role::System);
    assert_eq!(req.messages[1].role, Role::User);
}

#[test]
fn builder_optional_fields_absent_by_default() {
    let req = ChatRequest::builder("llama3.2").user("Hello!").build();

    assert!(req.temperature.is_none());
    assert!(req.max_tokens.is_none());
    assert!(req.top_p.is_none());
    assert!(req.stop.is_none());
    assert!(req.seed.is_none());
    assert!(req.tools.is_none());
    assert!(req.tool_choice.is_none());
}

#[test]
fn builder_with_all_scalar_options() {
    let req = ChatRequest::builder("gpt-4o")
        .user("Hello!")
        .temperature(0.5)
        .max_tokens(128)
        .top_p(0.9)
        .seed(42)
        .build();

    assert_eq!(req.temperature, Some(0.5));
    assert_eq!(req.max_tokens, Some(128));
    assert_eq!(req.top_p, Some(0.9));
    assert_eq!(req.seed, Some(42));
}

#[test]
fn builder_multiple_stop_sequences() {
    let req = ChatRequest::builder("gpt-4o")
        .user("Hello!")
        .stop("###")
        .stop("END")
        .build();

    assert_eq!(req.stop, Some(vec!["###".to_string(), "END".to_string()]));
}

#[test]
fn builder_messages_bulk() {
    let history = vec![
        ChatMessage::user("First message"),
        ChatMessage::assistant("First reply"),
    ];

    let req = ChatRequest::builder("llama3.2")
        .messages(history)
        .user("Follow-up")
        .build();

    assert_eq!(req.messages.len(), 3);
    assert_eq!(req.messages[2].content, "Follow-up");
}

#[test]
fn optional_fields_omitted_from_json() {
    let req = ChatRequest::builder("llama3.2").user("Hello!").build();

    let json = serde_json::to_string(&req).unwrap();

    assert!(!json.contains("temperature"));
    assert!(!json.contains("max_tokens"));
    assert!(!json.contains("top_p"));
    assert!(!json.contains("stop"));
    assert!(!json.contains("seed"));
    assert!(!json.contains("tools"));
    assert!(!json.contains("tool_choice"));
}

#[test]
fn roundtrip_serialization() {
    let original = ChatRequest::builder("gpt-4o")
        .system("Be concise.")
        .user("Hello!")
        .temperature(0.7)
        .max_tokens(256)
        .build();

    let json = serde_json::to_string(&original).unwrap();
    let restored: ChatRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}
