pub mod protocol;
pub mod provider;

pub mod prelude {
    pub use crate::protocol::{
        ChatMessage, ChatRequest, ChatResponse, ChatStreamChunk, Role, Tool, ToolCall, ToolChoice,
        Usage,
    };

    pub use crate::provider::{ChatStream, LlmProvider, Streamable};
}
