use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlmConfig {
    #[serde(default)]
    pub current_provider: LlmProvider,
    #[serde(default)]
    pub deepseek: DeepseekConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LlmProvider {
    #[default]
    Deepseek,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepseekConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_deepseek_base_url")]
    pub base_url: String,
    #[serde(default = "default_deepseek_model")]
    pub model: String,
}

impl Default for DeepseekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: default_deepseek_base_url(),
            model: default_deepseek_model(),
        }
    }
}

fn default_deepseek_base_url() -> String {
    "https://api.deepseek.com".to_string()
}

fn default_deepseek_model() -> String {
    "deepseek-chat".to_string()
}
