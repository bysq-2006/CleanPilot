use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlmConfig {
    #[serde(default)]
    pub current_provider: LlmProvider,
    #[serde(default)]
    pub deepseek: DeepseekConfig,
    #[serde(default)]
    pub openai: OpenAiConfig,
    #[serde(default)]
    pub openrouter: OpenRouterConfig,
    #[serde(default)]
    pub groq: GroqConfig,
    #[serde(default)]
    pub together: TogetherConfig,
    #[serde(default)]
    pub fireworks: FireworksConfig,
    #[serde(default)]
    pub moonshot: MoonshotConfig,
    #[serde(default)]
    pub zhipu: ZhipuConfig,
    #[serde(default)]
    pub dashscope: DashScopeConfig,
    #[serde(default)]
    pub siliconflow: SiliconFlowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LlmProvider {
    #[default]
    Deepseek,
    Openai,
    Openrouter,
    Groq,
    Together,
    Fireworks,
    Moonshot,
    Zhipu,
    Dashscope,
    Siliconflow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepseekConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for DeepseekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.deepseek.com".to_string(),
            model: "deepseek-chat".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for OpenAiConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4.1-mini".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            model: "openai/gpt-4.1-mini".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for GroqConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            model: "llama-3.3-70b-versatile".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TogetherConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for TogetherConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.together.xyz/v1".to_string(),
            model: "meta-llama/Llama-3.3-70B-Instruct-Turbo".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireworksConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for FireworksConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.fireworks.ai/inference/v1".to_string(),
            model: "accounts/fireworks/models/llama4-maverick-instruct-basic".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonshotConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for MoonshotConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.moonshot.cn/v1".to_string(),
            model: "moonshot-v1-8k".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhipuConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for ZhipuConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
            model: "glm-4-plus".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashScopeConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for DashScopeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".to_string(),
            model: "qwen-plus".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiliconFlowConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for SiliconFlowConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.siliconflow.cn/v1".to_string(),
            model: "Qwen/Qwen2.5-72B-Instruct".to_string(),
        }
    }
}
