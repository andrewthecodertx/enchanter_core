//! # Enchanter Agent

use crate::provider::{LlmProvider, OpenAiProvider, Provider};
use crate::tools::{ToolContext, ToolRegistry};
use crate::tools::tool::Tool as ToolTrait;
use crate::protocol::{ChatMessage, ChatRequest, ChatResponse};
use std::sync::Arc;

pub struct Agent {
    provider: Box<dyn LlmProvider>,
    model: String,
    registry: ToolRegistry,
    context: Arc<ToolContext>,
}

impl Agent {
    pub fn from_env() -> anyhow::Result<Self> {
        let provider = Provider::from_env();
        Self::from_provider(provider)
    }

    pub fn from_provider(provider: Provider) -> anyhow::Result<Self> {
        let provider = OpenAiProvider::from_env(provider)?;
        Self::from_openai(provider)
    }

    pub fn from_openai(provider: OpenAiProvider) -> anyhow::Result<Self> {
        let model = provider.default_model().to_string();
        Ok(Self {
            provider: Box::new(provider),
            model,
            registry: ToolRegistry::new(),
            context: Arc::new(ToolContext::new()),
        })
    }

    pub fn register_tool<T>(&mut self, tool: T)
    where
        T: ToolTrait + 'static,
    {
        self.registry.register(tool);
    }

    pub async fn chat(&self, prompt: impl Into<String>) -> anyhow::Result<ChatResponse> {
        let messages = vec![ChatMessage::user(prompt)];
        self.run_turn(messages).await
    }

    fn build_request(&self, messages: Vec<ChatMessage>) -> ChatRequest {
        let tools = self.registry.to_protocol_tools();
        let tool_choice = if tools.is_empty() {
            crate::protocol::ToolChoice::None
        } else {
            crate::protocol::ToolChoice::Auto
        };

        ChatRequest::builder(self.model.clone())
            .messages(messages)
            .tools(tools)
            .tool_choice(tool_choice)
            .build()
    }

    async fn run_turn(&self, messages: Vec<ChatMessage>) -> anyhow::Result<ChatResponse> {
        let provider = self.provider.as_ref();
        let request = self.build_request(messages);
        let response = provider.complete(request).await?;

        if !response.is_tool_call() {
            return Ok(response);
        }

        let mut updated = Vec::<ChatMessage>::new();
        updated.push(ChatMessage::assistant(response.message.content.clone()));

        for call in response.tool_calls.clone() {
            match self.registry.execute(&call, &self.context).await {
                Ok(result) => updated.push(result),
                Err(_) => continue,
            }
        }

        provider.complete(self.build_request(updated)).await
    }
}
