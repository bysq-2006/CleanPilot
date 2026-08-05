use std::sync::{Arc, Mutex};

use async_openai::types::ChatCompletionTool;
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

        match llm_config.current_provider {
            LlmProvider::Deepseek => {
                openai::chat_stream(
                    history,
                    &llm_config.deepseek.api_key,
                    &llm_config.deepseek.base_url,
                    &llm_config.deepseek.model,
                    tools,
                )
                .await
            }
            LlmProvider::Openai => {
                openai::chat_stream(
                    history,
                    &llm_config.openai.api_key,
                    &llm_config.openai.base_url,
                    &llm_config.openai.model,
                    tools,
                )
                .await
            }
            LlmProvider::Openrouter => {
                openai::chat_stream(
                    history,
                    &llm_config.openrouter.api_key,
                    &llm_config.openrouter.base_url,
                    &llm_config.openrouter.model,
                    tools,
                )
                .await
            }
            LlmProvider::Groq => {
                openai::chat_stream(
                    history,
                    &llm_config.groq.api_key,
                    &llm_config.groq.base_url,
                    &llm_config.groq.model,
                    tools,
                )
                .await
            }
            LlmProvider::Together => {
                openai::chat_stream(
                    history,
                    &llm_config.together.api_key,
                    &llm_config.together.base_url,
                    &llm_config.together.model,
                    tools,
                )
                .await
            }
            LlmProvider::Fireworks => {
                openai::chat_stream(
                    history,
                    &llm_config.fireworks.api_key,
                    &llm_config.fireworks.base_url,
                    &llm_config.fireworks.model,
                    tools,
                )
                .await
            }
            LlmProvider::Moonshot => {
                openai::chat_stream(
                    history,
                    &llm_config.moonshot.api_key,
                    &llm_config.moonshot.base_url,
                    &llm_config.moonshot.model,
                    tools,
                )
                .await
            }
            LlmProvider::Zhipu => {
                openai::chat_stream(
                    history,
                    &llm_config.zhipu.api_key,
                    &llm_config.zhipu.base_url,
                    &llm_config.zhipu.model,
                    tools,
                )
                .await
            }
            LlmProvider::Dashscope => {
                openai::chat_stream(
                    history,
                    &llm_config.dashscope.api_key,
                    &llm_config.dashscope.base_url,
                    &llm_config.dashscope.model,
                    tools,
                )
                .await
            }
            LlmProvider::Siliconflow => {
                openai::chat_stream(
                    history,
                    &llm_config.siliconflow.api_key,
                    &llm_config.siliconflow.base_url,
                    &llm_config.siliconflow.model,
                    tools,
                )
                .await
            }
            LlmProvider::Anthropic => {
                openai::chat_stream(
                    history,
                    &llm_config.anthropic.api_key,
                    &llm_config.anthropic.base_url,
                    &llm_config.anthropic.model,
                    tools,
                )
                .await
            }
            LlmProvider::Google => {
                openai::chat_stream(
                    history,
                    &llm_config.google.api_key,
                    &llm_config.google.base_url,
                    &llm_config.google.model,
                    tools,
                )
                .await
            }
            LlmProvider::Xai => {
                openai::chat_stream(
                    history,
                    &llm_config.xai.api_key,
                    &llm_config.xai.base_url,
                    &llm_config.xai.model,
                    tools,
                )
                .await
            }
            LlmProvider::Mistral => {
                openai::chat_stream(
                    history,
                    &llm_config.mistral.api_key,
                    &llm_config.mistral.base_url,
                    &llm_config.mistral.model,
                    tools,
                )
                .await
            }
            LlmProvider::Minimax => {
                openai::chat_stream(
                    history,
                    &llm_config.minimax.api_key,
                    &llm_config.minimax.base_url,
                    &llm_config.minimax.model,
                    tools,
                )
                .await
            }
            LlmProvider::Perplexity => {
                openai::chat_stream(
                    history,
                    &llm_config.perplexity.api_key,
                    &llm_config.perplexity.base_url,
                    &llm_config.perplexity.model,
                    tools,
                )
                .await
            }
            LlmProvider::Cerebras => {
                openai::chat_stream(
                    history,
                    &llm_config.cerebras.api_key,
                    &llm_config.cerebras.base_url,
                    &llm_config.cerebras.model,
                    tools,
                )
                .await
            }
            LlmProvider::Nvidia => {
                openai::chat_stream(
                    history,
                    &llm_config.nvidia.api_key,
                    &llm_config.nvidia.base_url,
                    &llm_config.nvidia.model,
                    tools,
                )
                .await
            }
            LlmProvider::Ollama => {
                openai::chat_stream(
                    history,
                    &llm_config.ollama.api_key,
                    &llm_config.ollama.base_url,
                    &llm_config.ollama.model,
                    tools,
                )
                .await
            }
            LlmProvider::Custom => {
                openai::chat_stream(
                    history,
                    &llm_config.custom.api_key,
                    &llm_config.custom.base_url,
                    &llm_config.custom.model,
                    tools,
                )
                .await
            }
        }
    }
}
