//! # Enchanter Tool

use crate::provider::LlmProvider;

pub struct Tool {
    provider: Box<dyn LlmProvider>,
}
