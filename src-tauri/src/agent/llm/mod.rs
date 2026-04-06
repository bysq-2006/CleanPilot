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
            LlmProvider::Deepseek => {
                openai::chat_stream(
                    history,
                    &llm_config.deepseek.api_key,
                    &llm_config.deepseek.base_url,
                    &llm_config.deepseek.model,
                )
                .await
            }
            LlmProvider::Openai => {
                openai::chat_stream(
                    history,
                    &llm_config.openai.api_key,
                    &llm_config.openai.base_url,
                    &llm_config.openai.model,
                )
                .await
            }
            LlmProvider::Openrouter => {
                openai::chat_stream(
                    history,
                    &llm_config.openrouter.api_key,
                    &llm_config.openrouter.base_url,
                    &llm_config.openrouter.model,
                )
                .await
            }
            LlmProvider::Groq => {
                openai::chat_stream(
                    history,
                    &llm_config.groq.api_key,
                    &llm_config.groq.base_url,
                    &llm_config.groq.model,
                )
                .await
            }
            LlmProvider::Together => {
                openai::chat_stream(
                    history,
                    &llm_config.together.api_key,
                    &llm_config.together.base_url,
                    &llm_config.together.model,
                )
                .await
            }
            LlmProvider::Fireworks => {
                openai::chat_stream(
                    history,
                    &llm_config.fireworks.api_key,
                    &llm_config.fireworks.base_url,
                    &llm_config.fireworks.model,
                )
                .await
            }
            LlmProvider::Moonshot => {
                openai::chat_stream(
                    history,
                    &llm_config.moonshot.api_key,
                    &llm_config.moonshot.base_url,
                    &llm_config.moonshot.model,
                )
                .await
            }
            LlmProvider::Zhipu => {
                openai::chat_stream(
                    history,
                    &llm_config.zhipu.api_key,
                    &llm_config.zhipu.base_url,
                    &llm_config.zhipu.model,
                )
                .await
            }
            LlmProvider::Dashscope => {
                openai::chat_stream(
                    history,
                    &llm_config.dashscope.api_key,
                    &llm_config.dashscope.base_url,
                    &llm_config.dashscope.model,
                )
                .await
            }
            LlmProvider::Siliconflow => {
                openai::chat_stream(
                    history,
                    &llm_config.siliconflow.api_key,
                    &llm_config.siliconflow.base_url,
                    &llm_config.siliconflow.model,
                )
                .await
            }
        }
    }
}
