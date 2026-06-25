//! # Enchanter Agent

use crate::provider::{ChatMessage, LlmProvider};
use anyhow::Result;

pub struct Agent {
    provider: Box<dyn LlmProvider>,
}
