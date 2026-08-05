use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl ProviderConfig {
    fn new(api_key: &str, base_url: &str, model: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(default)]
    pub current_provider: String,
    #[serde(flatten)]
    pub providers: HashMap<String, ProviderConfig>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            current_provider: "deepseek".to_string(),
            providers: default_providers(),
        }
    }
}

impl LlmConfig {
    /// 根据 current_provider 从哈希表中获取当前使用的模型配置。
    pub fn current_config(&self) -> Result<&ProviderConfig, String> {
        self.providers
            .get(&self.current_provider)
            .ok_or_else(|| format!("未找到当前 LLM 提供商配置: {}", self.current_provider))
    }
}

fn default_providers() -> HashMap<String, ProviderConfig> {
    HashMap::from([
        ("deepseek".into(), ProviderConfig::new("", "https://api.deepseek.com/v1", "deepseek-v4-flash")),
        ("openai".into(), ProviderConfig::new("", "https://api.openai.com/v1", "gpt-4.1-mini")),
        ("anthropic".into(), ProviderConfig::new("", "https://api.anthropic.com/v1", "claude-sonnet-4-6")),
        ("google".into(), ProviderConfig::new("", "https://generativelanguage.googleapis.com/v1beta/openai", "gemini-3.6-flash")),
        ("xai".into(), ProviderConfig::new("", "https://api.x.ai/v1", "grok-4.3")),
        ("openrouter".into(), ProviderConfig::new("", "https://openrouter.ai/api/v1", "openai/gpt-4.1-mini")),
        ("mistral".into(), ProviderConfig::new("", "https://api.mistral.ai/v1", "mistral-small-latest")),
        ("minimax".into(), ProviderConfig::new("", "https://api.minimaxi.com/v1", "MiniMax-M2.7")),
        ("perplexity".into(), ProviderConfig::new("", "https://api.perplexity.ai", "sonar-pro")),
        ("groq".into(), ProviderConfig::new("", "https://api.groq.com/openai/v1", "openai/gpt-oss-120b")),
        ("cerebras".into(), ProviderConfig::new("", "https://api.cerebras.ai/v1", "gpt-oss-120b")),
        ("nvidia".into(), ProviderConfig::new("", "https://integrate.api.nvidia.com/v1", "nvidia/llama-3.1-nemotron-nano-8b-v1")),
        ("together".into(), ProviderConfig::new("", "https://api.together.xyz/v1", "meta-llama/Llama-3.3-70B-Instruct-Turbo")),
        ("fireworks".into(), ProviderConfig::new("", "https://api.fireworks.ai/inference/v1", "accounts/fireworks/models/gpt-oss-120b")),
        ("moonshot".into(), ProviderConfig::new("", "https://api.moonshot.ai/v1", "kimi-k2.6")),
        ("zhipu".into(), ProviderConfig::new("", "https://open.bigmodel.cn/api/paas/v4", "glm-5.2")),
        ("dashscope".into(), ProviderConfig::new("", "https://dashscope.aliyuncs.com/compatible-mode/v1", "qwen3.6-plus")),
        ("siliconflow".into(), ProviderConfig::new("", "https://api.siliconflow.cn/v1", "Qwen/Qwen2.5-72B-Instruct")),
        ("ollama".into(), ProviderConfig::new("ollama", "http://localhost:11434/v1", "gpt-oss:20b")),
        ("custom".into(), ProviderConfig::new("", "", "")),
    ])
}
