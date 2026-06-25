//! # Enchanter Agent

use crate::provider::{ChatMessage, LlmProvider};
use anyhow::Result;

pub struct Tool {
    provider: Box<dyn LlmProvider>,
}

