pub mod providers;
pub mod types;

use self::types::LlmResponse;
use crate::models::llm_config::{LlmConfig, LlmProvider};

pub async fn chat(prompt: String, config: &LlmConfig) -> Result<LlmResponse, String> {
    match config.current_provider {
        LlmProvider::Deepseek => providers::deepseek::chat(prompt, &config.deepseek).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::llm_config::{DeepseekConfig, LlmConfig, LlmProvider};

    /// 运行方式（Windows cmd）:
    /// set DEEPSEEK_API_KEY=你的key && cargo test deepseek_chat_smoke -- --nocapture
    #[test]
    fn deepseek_chat_smoke() {
        let config = LlmConfig {
            current_provider: LlmProvider::Deepseek,
            deepseek: DeepseekConfig {
                api_key: "sk-a71c9a4a20014c6b83a9af47b7e2f2ae".to_string(),
                base_url: "https://api.deepseek.com".to_string(),
                model: "deepseek-chat".to_string(),
            },
        };

        let result = tauri::async_runtime::block_on(chat("请回复：连接成功".to_string(), &config));

        print!("result: {:?}", result);
    }
}
