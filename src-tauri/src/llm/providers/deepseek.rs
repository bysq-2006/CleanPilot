use crate::llm::types::LlmResponse;
use crate::models::llm_config::DeepseekConfig;
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

fn get_required_config(value: &str, key: &str) -> Result<String, String> {
    let value = value
        .trim()
        .to_string();

    if value.is_empty() {
        return Err(format!("missing required config: {key}"));
    }

    Ok(value)
}

/// 调用 DeepSeek 的 Chat Completions API
pub async fn chat(prompt: String, config: &DeepseekConfig) -> Result<LlmResponse, String> {
    let api_key = get_required_config(&config.api_key, "api_key")?;
    let model = config.model.trim().to_string();
    let base_url = config.base_url.trim().to_string();

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
