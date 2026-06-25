//! # Enchanter Agent

use crate::provider::LlmProvider;

pub struct Agent {
    provider: Box<dyn LlmProvider>,
}
