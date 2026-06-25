use crate::protocol::message::ChatMessage;
use crate::protocol::tool::{Tool, ToolChoice};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

impl ChatRequest {
    pub fn builder(model: impl Into<String>) -> ChatRequestBuilder {
        ChatRequestBuilder::new(model)
    }
}

#[derive(Debug)]
pub struct ChatRequestBuilder {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    top_p: Option<f32>,
    stop: Option<Vec<String>>,
    seed: Option<u64>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
}

impl ChatRequestBuilder {
    fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: Vec::new(),
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
            seed: None,
            tools: None,
            tool_choice: None,
        }
    }

    pub fn system(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::system(content));
        self
    }

    pub fn user(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::user(content));
        self
    }

    pub fn assistant(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::assistant(content));
        self
    }

    pub fn message(mut self, message: ChatMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn messages(mut self, messages: impl IntoIterator<Item = ChatMessage>) -> Self {
        self.messages.extend(messages);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn stop(mut self, sequence: impl Into<String>) -> Self {
        self.stop.get_or_insert_with(Vec::new).push(sequence.into());
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn tool(mut self, tool: Tool) -> Self {
        self.tools.get_or_insert_with(Vec::new).push(tool);
        self
    }

    pub fn tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    pub fn build(self) -> ChatRequest {
        ChatRequest {
            model: self.model,
            messages: self.messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            top_p: self.top_p,
            stop: self.stop,
            seed: self.seed,
            tools: self.tools,
            tool_choice: self.tool_choice,
        }
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::message::Role;

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
}
