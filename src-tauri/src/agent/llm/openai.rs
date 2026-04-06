use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, CreateChatCompletionStreamResponse,
};
use async_openai::Client;
use futures_util::{Stream, StreamExt};

use crate::agent::context::history::AgentHistory;
use super::utils::{get_required_config, normalize_openai_api_base};

pub type LlmStream =
    std::pin::Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, String>> + Send>>;

pub async fn chat_stream(
    history: &AgentHistory,
    api_key: &str,
    base_url: &str,
    model: &str,
) -> Result<LlmStream, String> {
    let api_key = get_required_config(api_key, "api_key")?;
    let base_url = get_required_config(base_url, "base_url")?;
    let model = get_required_config(model, "model")?;
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
        .map_err(|e| format!("openai user message build failed: {e}"))?;

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([message])
        .stream(true)
        .build()
        .map_err(|e| format!("openai request build failed: {e}"))?;

    let stream = client
        .chat()
        .create_stream(request)
        .await
        .map_err(|e| format!("failed to call openai-compatible api: {e}"))?;

    Ok(Box::pin(stream.map(|chunk| {
        chunk.map_err(|e| format!("openai stream chunk failed: {e}"))
    })))
}
