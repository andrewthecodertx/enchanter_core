use crate::provider::{ChatMessage, ChatRequest, Role};

#[test]
fn test_message_constructors() {
    let sys = ChatMessage::system("You are helpful.");
    let usr = ChatMessage::user("Hello!");
    let ast = ChatMessage::assistant("Hi there!");
    let tol = ChatMessage::tool("result");

    assert_eq!(sys.role, Role::System);
    assert_eq!(usr.role, Role::User);
    assert_eq!(ast.role, Role::Assistant);
    assert_eq!(tol.role, Role::Tool);
    assert_eq!(usr.content, "Hello!");
}

#[test]
fn test_chat_request_defaults() {
    let req = ChatRequest::new("test-model", vec![ChatMessage::user("Hello!")]);

    assert_eq!(req.model, "test-model");
    assert_eq!(req.messages.len(), 1);
    assert!(req.temperature.is_none());
    assert!(req.max_tokens.is_none());
}

#[test]
fn test_role_serializes_lowercase() {
    assert_eq!(serde_json::to_string(&Role::User).unwrap(), "\"user\"");
    assert_eq!(serde_json::to_string(&Role::System).unwrap(), "\"system\"");
    assert_eq!(
        serde_json::to_string(&Role::Assistant).unwrap(),
        "\"assistant\""
    );
    assert_eq!(serde_json::to_string(&Role::Tool).unwrap(), "\"tool\"");
}

#[test]
fn test_chat_request_roundtrip() {
    let req = ChatRequest::new("test-model", vec![ChatMessage::user("Hello!")]);

    let json = serde_json::to_string(&req).unwrap();
    let decoded: ChatRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, req);
}

#[test]
fn test_optional_fields_omitted_from_json() {
    let req = ChatRequest::new("test-model", vec![ChatMessage::user("Hello!")]);

    let json = serde_json::to_string(&req).unwrap();

    assert!(!json.contains("temperature"));
    assert!(!json.contains("max_tokens"));
}
