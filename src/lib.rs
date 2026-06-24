//! # Enchanter CORE

/// LLM provider interface.
pub mod provider;

/// extensible tool system.
pub mod tool;

/// the agent orchestration logic.
pub mod agent;

/// preliminary convenience re-exports.
pub mod prelude {
    pub use crate::agent::Agent;
    pub use crate::provider::LlmProvider;
    pub use crate::tool::Tool;
}
