use crate::protocol::message::{ChatMessage, Role};

#[test]
fn role_serializes_lowercase() {
    assert_eq!(serde_json::to_string(&Role::User).unwrap(), r#""user""#);
    assert_eq!(
        serde_json::to_string(&Role::Assistant).unwrap(),
        r#""assistant""#
    );
}

#[test]
fn role_display() {
    assert_eq!(Role::System.to_string(), "system");
    assert_eq!(Role::User.to_string(), "user");
    assert_eq!(Role::Assistant.to_string(), "assistant");
    assert_eq!(Role::Tool.to_string(), "tool");
}

#[test]
fn convenience_constructors() {
    let msg = ChatMessage::user("Hello!");
    assert_eq!(msg.role, Role::User);
    assert_eq!(msg.content, "Hello!");
    assert!(msg.tool_call_id.is_none());
}

#[test]
fn tool_message_carries_call_id() {
    let msg = ChatMessage::tool("call_abc123", r#"{"result": 42}"#);
    assert_eq!(msg.role, Role::Tool);
    assert_eq!(msg.tool_call_id.as_deref(), Some("call_abc123"));
}

#[test]
fn tool_call_id_omitted_when_none() {
    let msg = ChatMessage::user("Hello!");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(!json.contains("tool_call_id"));
}

#[test]
fn roundtrip_serialization() {
    let original = ChatMessage::assistant("Hi there!");
    let json = serde_json::to_string(&original).unwrap();
    let restored: ChatMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}
