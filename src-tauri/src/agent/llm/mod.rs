use std::sync::{Arc, Mutex};

use async_openai::types::ChatCompletionTool;
use crate::agent::context::history::AgentHistory;
use crate::models::config::Config;

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

    pub async fn chat_stream(
        &self,
        history: &AgentHistory,
        tools: Vec<ChatCompletionTool>,
    ) -> Result<openai::LlmStream, String> {
        let llm_config = self
            .config
            .lock()
            .map_err(|e| format!("LLM 配置锁获取失败: {}", e))?
            .llm
            .clone();

        let provider = llm_config.current_config()?;
        openai::chat_stream(
            history,
            &provider.api_key,
            &provider.base_url,
            &provider.model,
            tools,
        )
        .await
    }
}
