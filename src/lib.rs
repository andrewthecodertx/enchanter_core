pub mod protocol;
pub mod provider;
pub mod tools;
pub mod agent;

pub mod prelude {
    pub use crate::agent::Agent;
    pub use crate::protocol::{
        ChatMessage, ChatRequest, ChatResponse, ChatStreamChunk, Role, Tool, ToolCall, ToolChoice,
        Usage,
    };
    pub use crate::provider::{ChatStream, LlmProvider, OpenAiProvider, Provider, Streamable};
    pub use crate::tools::{ToolContext, ToolError, ToolRegistry};
}
