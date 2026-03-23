use crate::agent::history::{AgentMessage, AgentToolCall};
use crate::agent::runtime::AgentRuntime;
use crate::agent::task_queue::AgentTask;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AgentAssistantMessage {
    #[serde(rename = "type")]
    message_type: String,
    content: Option<String>,
    tool_calls: Option<Vec<AgentToolCall>>,
}

/// 处理用户问题任务：先写入 user 消息，再把 LLM 原始输出交给 agent 侧解析器处理。
pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
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

    println!("Agent 收到用户问题任务: {}", content);
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
    match runtime.llm.chat(&runtime.history).await {
        Ok(raw_reply) => match parse_llm_tasks(&raw_reply) {
            Ok(tasks) => {
                enqueue_tasks(runtime, tasks, enqueue_error_prefix);
                println!("{}: {}", success_log, raw_reply);
            }
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("LLM 调用失败: {}", e),
    }
}

fn enqueue_tasks(runtime: &AgentRuntime, tasks: Vec<AgentTask>, error_prefix: &str) {
    for task in tasks {
        if let Err(e) = runtime.tasks.push(task) {
            eprintln!("{}: {}", error_prefix, e);
            break;
        }
    }
}

/// 解析标准 assistant 消息；如果存在 tool_calls，则工具执行完后再触发下一轮。
pub fn parse_llm_tasks(raw: &str) -> Result<Vec<AgentTask>, String> {
    let message: AgentAssistantMessage = serde_json::from_str(raw).map_err(|e| {
        format!(
            "LLM 返回 assistant 消息解析失败: {}\n原始输出: {}",
            e, raw
        )
    })?;

    if message.message_type != "assistant" {
        return Err(format!("LLM 返回的 type 不是 assistant: {}", message.message_type));
    }

    let mut tasks = Vec::new();

    if let Some(content) = message.content {
        if !content.trim().is_empty() {
            tasks.push(AgentTask::AssistantReply { content });
        }
    }

    if let Some(tool_calls) = message.tool_calls {
        for tool_call in tool_calls {
            tasks.push(AgentTask::ToolCall {
                tool_call_id: tool_call.id,
                tool_name: tool_call.function.name,
                payload: tool_call.function.arguments,
            });
        }

        tasks.push(AgentTask::RunAgentLoop);
    }

    Ok(tasks)
}

