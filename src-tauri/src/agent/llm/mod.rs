use std::sync::{Arc, Mutex};

use crate::agent::context::history::AgentHistory;
use crate::models::config::Config;
use crate::models::llm_config::LlmProvider;

pub mod openai;
mod utils;

#[derive(Debug, Clone)]
pub struct AgentLlm {
    config: Arc<Mutex<Config>>,
}

impl AgentLlm {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        Self { config }
    }

    pub async fn chat_stream(&self, history: &AgentHistory) -> Result<openai::LlmStream, String> {
        let llm_config = self
            .config
            .lock()
            .map_err(|e| format!("LLM 配置锁获取失败: {}", e))?
            .llm
            .clone();

        match llm_config.current_provider {
            LlmProvider::Deepseek => openai::chat_stream(history, &llm_config.deepseek).await,
        }
    }
}
