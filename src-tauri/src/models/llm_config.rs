use serde::{Deserialize, Serialize};

macro_rules! provider_config {
    ($name:ident, $api_key:expr, $base_url:expr, $model:expr) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            pub api_key: String,
            pub base_url: String,
            pub model: String,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    api_key: $api_key.to_string(),
                    base_url: $base_url.to_string(),
                    model: $model.to_string(),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlmConfig {
    #[serde(default)]
    pub current_provider: LlmProvider,
    #[serde(default)]
    pub deepseek: DeepseekConfig,
    #[serde(default)]
    pub openai: OpenAiConfig,
    #[serde(default)]
    pub anthropic: AnthropicConfig,
    #[serde(default)]
    pub google: GoogleConfig,
    #[serde(default)]
    pub xai: XAiConfig,
    #[serde(default)]
    pub openrouter: OpenRouterConfig,
    #[serde(default)]
    pub mistral: MistralConfig,
    #[serde(default)]
    pub minimax: MiniMaxConfig,
    #[serde(default)]
    pub perplexity: PerplexityConfig,
    #[serde(default)]
    pub groq: GroqConfig,
    #[serde(default)]
    pub cerebras: CerebrasConfig,
    #[serde(default)]
    pub nvidia: NvidiaConfig,
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
    #[serde(default)]
    pub ollama: OllamaConfig,
    #[serde(default)]
    pub custom: CustomConfig,
}

impl LlmConfig {
    pub fn migrate_legacy_defaults(&mut self) {
        if self.deepseek.model == "deepseek-chat" {
            self.deepseek.model = "deepseek-v4-flash".to_string();
        }
        if self.deepseek.base_url == "https://api.deepseek.com" {
            self.deepseek.base_url = "https://api.deepseek.com/v1".to_string();
        }
        if self.groq.model == "llama-3.3-70b-versatile" {
            self.groq.model = "openai/gpt-oss-120b".to_string();
        }
        if self.fireworks.model == "accounts/fireworks/models/llama4-maverick-instruct-basic" {
            self.fireworks.model = "accounts/fireworks/models/gpt-oss-120b".to_string();
        }
        if self.moonshot.model == "moonshot-v1-8k" {
            self.moonshot.base_url = "https://api.moonshot.ai/v1".to_string();
            self.moonshot.model = "kimi-k2.6".to_string();
        }
        if self.zhipu.model == "glm-4-plus" {
            self.zhipu.model = "glm-5.2".to_string();
        }
        if self.dashscope.model == "qwen-plus" {
            self.dashscope.model = "qwen3.6-plus".to_string();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LlmProvider {
    #[default]
    Deepseek,
    Openai,
    Anthropic,
    Google,
    Xai,
    Openrouter,
    Mistral,
    Minimax,
    Perplexity,
    Groq,
    Cerebras,
    Nvidia,
    Together,
    Fireworks,
    Moonshot,
    Zhipu,
    Dashscope,
    Siliconflow,
    Ollama,
    Custom,
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
            base_url: "https://api.deepseek.com/v1".to_string(),
            model: "deepseek-v4-flash".to_string(),
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
            model: "openai/gpt-oss-120b".to_string(),
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
            model: "accounts/fireworks/models/gpt-oss-120b".to_string(),
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
            base_url: "https://api.moonshot.ai/v1".to_string(),
            model: "kimi-k2.6".to_string(),
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
            model: "glm-5.2".to_string(),
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
            model: "qwen3.6-plus".to_string(),
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

provider_config!(
    AnthropicConfig,
    "",
    "https://api.anthropic.com/v1",
    "claude-sonnet-4-6"
);
provider_config!(
    GoogleConfig,
    "",
    "https://generativelanguage.googleapis.com/v1beta/openai",
    "gemini-3.6-flash"
);
provider_config!(XAiConfig, "", "https://api.x.ai/v1", "grok-4.3");
provider_config!(
    MistralConfig,
    "",
    "https://api.mistral.ai/v1",
    "mistral-small-latest"
);
provider_config!(
    MiniMaxConfig,
    "",
    "https://api.minimaxi.com/v1",
    "MiniMax-M2.7"
);
provider_config!(
    PerplexityConfig,
    "",
    "https://api.perplexity.ai",
    "sonar-pro"
);
provider_config!(
    CerebrasConfig,
    "",
    "https://api.cerebras.ai/v1",
    "gpt-oss-120b"
);
provider_config!(
    NvidiaConfig,
    "",
    "https://integrate.api.nvidia.com/v1",
    "nvidia/llama-3.1-nemotron-nano-8b-v1"
);
provider_config!(
    OllamaConfig,
    "ollama",
    "http://localhost:11434/v1",
    "gpt-oss:20b"
);
provider_config!(CustomConfig, "", "", "");
