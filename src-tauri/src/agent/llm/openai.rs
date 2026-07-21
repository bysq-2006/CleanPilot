use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionTool, ChatCompletionToolType, CreateChatCompletionRequestArgs,
    CreateChatCompletionStreamResponse, FunctionCall,
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
    tools: Vec<ChatCompletionTool>,
) -> Result<LlmStream, String> {
    let api_key = get_required_config(api_key, "api_key")?;
    let base_url = get_required_config(base_url, "base_url")?;
    let model = get_required_config(model, "model")?;
    let api_base = normalize_openai_api_base(&base_url);

    let openai_config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(api_base);

    let client = Client::with_config(openai_config);
    let mut request = CreateChatCompletionRequestArgs::default();
    request
        .model(model)
        .messages(build_messages(history)?)
        .stream(true);
    if !tools.is_empty() {
        request.tools(tools);
    }
    let request = request.build()
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

fn build_messages(history: &AgentHistory) -> Result<Vec<ChatCompletionRequestMessage>, String> {
    let system = history
        .system_prompt
        .lock()
        .map_err(|e| format!("Agent system prompt 锁获取失败: {e}"))?
        .build();
    let history = history
        .inner
        .lock()
        .map_err(|e| format!("Agent 历史记录锁获取失败: {e}"))?;

    let mut messages = vec![ChatCompletionRequestMessage::System(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system)
            .build()
            .map_err(|e| e.to_string())?,
    )];

    for message in history.iter() {
        let content = message.content.clone().unwrap_or_default();
        let converted = match message.role.as_str() {
            "user" => ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()
                    .map_err(|e| e.to_string())?,
            ),
            "assistant" => {
                let calls = message.tool_calls.as_ref().map(|calls| {
                    calls
                        .iter()
                        .map(|call| ChatCompletionMessageToolCall {
                            id: call.id.clone(),
                            r#type: ChatCompletionToolType::Function,
                            function: FunctionCall {
                                name: call.function.name.clone(),
                                arguments: call.function.arguments.clone(),
                            },
                        })
                        .collect::<Vec<_>>()
                });
                let mut assistant = ChatCompletionRequestAssistantMessageArgs::default();
                assistant.content(content);
                if let Some(calls) = calls {
                    assistant.tool_calls(calls);
                }
                ChatCompletionRequestMessage::Assistant(
                    assistant
                        .build()
                        .map_err(|e| e.to_string())?,
                )
            }
            "tool" => ChatCompletionRequestMessage::Tool(
                ChatCompletionRequestToolMessageArgs::default()
                    .content(content)
                    .tool_call_id(
                        message
                            .tool_call_id
                            .clone()
                            .ok_or_else(|| "tool 消息缺少 tool_call_id".to_string())?,
                    )
                    .build()
                    .map_err(|e| e.to_string())?,
            ),
            role => return Err(format!("不支持的消息角色: {role}")),
        };
        messages.push(converted);
    }

    Ok(messages)
}
