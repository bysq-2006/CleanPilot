pub mod providers;
pub mod types;

use self::types::{LlmConfigMap, LlmResponse};

pub async fn chat(provider: String, prompt: String, config: LlmConfigMap) -> Result<LlmResponse, String> {
    match provider.as_str() {
        "deepseek" => providers::deepseek::chat(prompt, config).await,
        _ => Err(format!("unsupported provider: {}", provider)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// 运行方式（Windows cmd）:
    /// set DEEPSEEK_API_KEY=你的key && cargo test deepseek_chat_smoke -- --nocapture
    #[test]
    fn deepseek_chat_smoke() {

        let mut config = HashMap::new();
        config.insert("api_key".to_string(), "sk-a71c9a4a20014c6b83a9af47b7e2f2ae".to_string());
        config.insert("model".to_string(), "deepseek-chat".to_string());

        let result = tauri::async_runtime::block_on(chat(
            "deepseek".to_string(),
            "请回复：连接成功".to_string(),
            config,
        ));

        print!("result: {:?}", result);
    }
}
