use crate::protocol::{ChatMessage, ChatRequest, ChatResponse, Role, ToolCall, Usage};
use crate::provider::LlmProvider;
use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAiProvider {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl OpenAiProvider {
    pub fn new(base_url: impl Into<String>, api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            api_key,
        }
    }

    pub fn custom(base_url: impl Into<String>, api_key: impl Into<Option<String>>) -> Self {
        Self::new(base_url, api_key.into())
    }

    pub fn openai() -> anyhow::Result<Self> {
        let key = std::env::var("API_KEY").context("API_KEY environment variable not set")?;
        Ok(Self::new("https://api.openai.com/v1", Some(key)))
    }

    pub fn ollama() -> Self {
        Self::new("http://localhost:11434/v1", None)
    }

    pub fn lm_studio() -> Self {
        Self::new("http://localhost:1234/v1", None)
    }

    pub fn from_env(provider: Provider) -> anyhow::Result<Self> {
        let provider = match provider {
            Provider::OpenAi => Self::openai()?,
            Provider::Ollama => Self::ollama(),
            Provider::LmStudio => Self::lm_studio(),
            Provider::Custom => {
                let base_url = std::env::var("ENCHANTER_BASE_URL").context(
                    "ENCHANTER_BASE_URL must be set when using provider=custom",
                )?;
                let api_key = std::env::var("ENCHANTER_API_KEY").ok();
                Self::custom(base_url, api_key)
            }
        };

        Ok(provider)
    }

    pub fn default_model(&self) -> &'static str {
        if self.base_url.contains("openai.com") {
            "gpt-4o-mini"
        } else if self.base_url.contains("11434") {
            "gemma4:12b"
        } else if self.base_url.contains("1234") {
            "local-model"
        } else {
            "gpt-4o-mini"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    OpenAi,
    Ollama,
    LmStudio,
    Custom,
}

impl Provider {
    pub fn from_env() -> Self {
        std::env::var("ENCHANTER_PROVIDER")
            .ok()
            .and_then(|value| match value.to_lowercase().as_str() {
                "openai" => Some(Provider::OpenAi),
                "ollama" => Some(Provider::Ollama),
                "lmstudio" | "lm_studio" | "lm-studio" => Some(Provider::LmStudio),
                "custom" => Some(Provider::Custom),
                _ => None,
            })
            .unwrap_or(Provider::Ollama)
    }
}

#[derive(Serialize)]
struct ApiRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,

    stream: bool,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<ApiChoice>,
    usage: Option<ApiUsage>,
}

#[derive(Deserialize)]
struct ApiChoice {
    message: ApiMessage,
    finish_reason: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ToolCall>,
}

#[derive(Deserialize)]
struct ApiMessage {
    role: Role,
    content: String,
}

#[derive(Deserialize)]
struct ApiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn complete(&self, request: ChatRequest) -> anyhow::Result<ChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = ApiRequest {
            model: &request.model,
            messages: &request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: false,
        };

        let mut req = self.client.post(&url).json(&body);

        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }

        let api_res = req
            .send()
            .await
            .context("failed to send request to provider")?
            .error_for_status()
            .context("provider returned an error status")?
            .json::<ApiResponse>()
            .await
            .context("failed to deserialize provider response")?;

        let choice = api_res
            .choices
            .into_iter()
            .next()
            .context("provider returned no choices")?;

        Ok(ChatResponse {
            message: ChatMessage {
                role: choice.message.role,
                content: choice.message.content,
                tool_call_id: None,
            },
            finish_reason: choice.finish_reason,
            usage: api_res.usage.map(|u| Usage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
            tool_calls: choice.tool_calls,
        })
    }
}
