use crate::llm::types::{LlmConfigMap, LlmResponse};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
}

#[derive(Debug, Serialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekAssistantMessage,
}

#[derive(Debug, Deserialize)]
struct DeepSeekAssistantMessage {
    content: String,
}

fn get_required_config(config: &LlmConfigMap, key: &str) -> Result<String, String> {
    let value = config
        .get(key)
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("missing required config: {key}"))?;

    Ok(value.to_string())
}

fn get_optional_config(config: &LlmConfigMap, key: &str, default: &str) -> String {
    config
        .get(key)
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .unwrap_or(default)
        .to_string()
}

/// 调用 DeepSeek 的 Chat Completions API
/// config 需要有：
/// 1. api_key: DeepSeek API Key
/// 2. model: 可选，默认为 "deepseek-chat"
pub async fn chat(prompt: String, config: LlmConfigMap) -> Result<LlmResponse, String> {
    let api_key = get_required_config(&config, "api_key")?;
    let model = get_optional_config(&config, "model", "deepseek-chat");
    let base_url = get_optional_config(&config, "base_url", "https://api.deepseek.com");

    let endpoint = format!("{}/chat/completions", base_url.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let req_body = DeepSeekRequest {
        model: model.clone(),
        messages: vec![DeepSeekMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let response = client
        .post(endpoint)
        .header(AUTHORIZATION, format!("Bearer {api_key}"))
        .header(CONTENT_TYPE, "application/json")
        .json(&req_body)
        .send()
        .await
        .map_err(|e| format!("failed to call deepseek api: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response
            .text()
            .await
            .unwrap_or_else(|_| "<failed to read error body>".to_string());
        return Err(format!("deepseek api returned {status}: {body_text}"));
    }

    let data = response
        .json::<DeepSeekResponse>()
        .await
        .map_err(|e| format!("failed to parse deepseek response: {e}"))?;

    let content = data
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| "deepseek response has no choices".to_string())?;

    Ok(LlmResponse::new("deepseek", model, content))
}
