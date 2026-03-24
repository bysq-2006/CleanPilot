use std::sync::{Arc, Mutex};

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;

use crate::agent::context::history::AgentHistory;
use crate::models::config::Config;
use crate::models::llm_config::LlmProvider;

#[derive(Debug, Clone)]
pub struct AgentLlm {
    config: Arc<Mutex<Config>>,
}

impl AgentLlm {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        Self { config }
    }

    /// 基于当前内部配置调用聊天接口；当前先保留非流式。
    pub async fn chat(&self, history: &AgentHistory) -> Result<String, String> {
        let llm_config = self
            .config
            .lock()
            .map_err(|e| format!("LLM 配置锁获取失败: {}", e))?
            .llm
            .clone();

        match llm_config.current_provider {
            LlmProvider::Deepseek => self.chat_with_deepseek(history, &llm_config.deepseek).await,
        }
    }

    async fn chat_with_deepseek(
        &self,
        history: &AgentHistory,
        config: &crate::models::llm_config::DeepseekConfig,
    ) -> Result<String, String> {
        let api_key = get_required_config(&config.api_key, "api_key")?;
        let base_url = get_required_config(&config.base_url, "base_url")?;
        let model = get_required_config(&config.model, "model")?;
        let llm_input = history.build_llm_input()?;
        let api_base = normalize_openai_api_base(&base_url);

        let openai_config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base);

        let client = Client::with_config(openai_config);
        let message: ChatCompletionRequestMessage = ChatCompletionRequestUserMessageArgs::default()
            .content(llm_input)
            .build()
            .map(ChatCompletionRequestMessage::User)
            .map_err(|e| format!("deepseek user message build failed: {e}"))?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages([message])
            .build()
            .map_err(|e| format!("deepseek request build failed: {e}"))?;

        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| format!("failed to call deepseek api: {e}"))?;

        response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| "deepseek response has no content".to_string())
    }
}

fn get_required_config(value: &str, key: &str) -> Result<String, String> {
    let value = value.trim().to_string();

    if value.is_empty() {
        return Err(format!("missing required config: {key}"));
    }

    Ok(value)
}

fn normalize_openai_api_base(base_url: &str) -> String {
    let trimmed = base_url.trim_end_matches('/');

    if trimmed.ends_with("/v1") {
        trimmed.to_string()
    }
    else {
        format!("{trimmed}/v1")
    }
}
