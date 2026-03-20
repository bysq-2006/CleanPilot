pub mod providers;
pub mod types;

use self::types::LlmResponse;
use crate::models::llm_config::{LlmConfig, LlmProvider};

/// 通过config匹配不同的LLM提供商，并调用对应的chat函数
pub async fn chat(prompt: String, config: &LlmConfig) -> Result<LlmResponse, String> {
    match config.current_provider {
        LlmProvider::Deepseek => providers::deepseek::chat(prompt, &config.deepseek).await,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deepseek_chat_smoke() {
    }
}
