use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmResponse {
    pub provider: String,
    pub model: String,
    pub content: String,
}

impl LlmResponse {
    pub fn new(provider: impl Into<String>, model: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            model: model.into(),
            content: content.into(),
        }
    }
}
