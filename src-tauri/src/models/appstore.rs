use std::sync::Mutex;

use super::config::Config;
use crate::llm;
use crate::llm::types::LlmResponse;

/// 全局状态管理，存储一些全局共享的数据
#[derive(Default)]
pub struct AppStore {
    pub config: Mutex<Config>,
    pub chat_history: Mutex<Vec<String>>,
}

impl AppStore {
    /// 这一层是为了不要每次调用聊天接口的时候都要手动传一次配置。
    /// 不在 LLM 里面直接做这个传配置的操作，是因为那边要获取 APP store 特别麻烦
    pub async fn chat(&self, prompt: String) -> Result<LlmResponse, String> {
        let llm_config = {
            let config = self
                .config
                .lock()
                .map_err(|e| format!("配置锁获取失败: {}", e))?;
            config.llm.clone()
        };

        llm::chat(prompt, &llm_config).await
    }
}
