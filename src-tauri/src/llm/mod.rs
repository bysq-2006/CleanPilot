pub mod providers;

use std::sync::{Arc, Mutex};

use crate::agent::history::AgentMessage;
use crate::models::config::Config;
use crate::models::llm_config::LlmProvider;

#[derive(Debug, Clone)]
pub struct LlmService {
    config: Arc<Mutex<Config>>,
}

impl LlmService {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        Self { config }
    }

    /// 基于当前内部配置调用聊天接口
    pub async fn chat(&self, messages: &[AgentMessage]) -> Result<String, String> {
        let llm_config = self
            .config
            .lock()
            .map_err(|e| format!("LLM 配置锁获取失败: {}", e))?
            .llm
            .clone();

        match llm_config.current_provider {
            LlmProvider::Deepseek => providers::deepseek::chat(messages, &llm_config.deepseek).await,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deepseek_chat_smoke() {
    }
}
