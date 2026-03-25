use crate::agent::context::history::{AgentMessage, AgentToolCall};
use crate::agent::runtime::AgentRuntime;
use crate::agent::tasks::queue::AgentTask;
use futures_util::StreamExt;
use serde::Deserialize

#[derive(Debug, Deserialize)]
struct AgentAssistantMessage {
    #[serde(rename = "type")]
    message_type: String,
    content: Option<String>,
    tool_calls: Option<Vec<AgentToolCall>>,
}

/// 处理用户问题任务：先写入 user 消息，再把 LLM 原始输出交给 agent 侧解析器处理。
pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
    println!("Agent 收到用户问题任务: {}", content);
    
    if let Err(e) = runtime.history.append(AgentMessage {
        role: "user".to_string(),
        content: Some(content.clone()),
        tool_calls: None,
        tool_call_id: None,
    }) {
        eprintln!("Agent 写入历史记录失败: {}", e);
    }

    request_and_enqueue_tasks(
        runtime,
        "Agent 已根据用户问题生成任务列表",
        "Agent 任务入队失败",
    )
    .await;
}

/// 工具执行后继续请求 LLM，provider 仍然只返回原始字符串。
pub async fn handle_continue_reply(runtime: &AgentRuntime) {
    request_and_enqueue_tasks(
        runtime,
        "Agent 已根据历史记录生成后续任务列表",
        "Agent 继续回答任务入队失败",
    )
    .await;
}

async fn request_and_enqueue_tasks(
    runtime: &AgentRuntime,
    success_log: &str,
    enqueue_error_prefix: &str,
) {
    match runtime.llm.chat_stream(&runtime.history).await {
        Ok(mut stream) => {
        }
        Err(e) => eprintln!("LLM 调用失败: {}", e),
    }
}