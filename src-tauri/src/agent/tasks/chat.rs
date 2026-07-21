use std::collections::BTreeMap;

use futures_util::StreamExt;

use crate::agent::context::history::{AgentMessage, AgentToolCall, AgentToolFunction};
use crate::agent::runtime::{AgentRuntime, AgentStatus};
use crate::agent::tasks::queue::AgentTask;

pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
    if let Err(e) = runtime.set_status(AgentStatus::Chatting) {
        eprintln!("Agent 切换到聊天状态失败: {e}");
    }

    if let Err(e) = runtime.history.append(AgentMessage {
        role: "user".to_string(),
        content: Some(content),
        tool_name: None,
        tool_calls: None,
        tool_call_id: None,
    }) {
        eprintln!("Agent 写入用户消息失败: {e}");
        return;
    }

    request_and_enqueue_tasks(runtime).await;
}

pub async fn handle_continue_reply(runtime: &AgentRuntime) {
    request_and_enqueue_tasks(runtime).await;
}

async fn request_and_enqueue_tasks(runtime: &AgentRuntime) {
    let tools = match runtime.tools.lock() {
        Ok(tools) => tools.api_tools(),
        Err(e) => return fail(runtime, format!("Agent 工具锁获取失败: {e}")),
    };
    let mut stream = match runtime.llm.chat_stream(&runtime.history, tools).await {
        Ok(stream) => stream,
        Err(e) => return fail(runtime, format!("LLM 调用失败: {e}")),
    };

    if let Err(e) = append_assistant(runtime) {
        return fail(runtime, e);
    }

    let mut calls = BTreeMap::<i32, AgentToolCall>::new();
    while let Some(chunk) = stream.next().await {
        let response = match chunk {
            Ok(response) => response,
            Err(e) => return fail(runtime, format!("LLM 流式响应失败: {e}")),
        };

        for choice in response.choices {
            let delta = choice.delta;
            if let Some(content) = delta.content.or(delta.refusal) {
                if let Err(e) = append_content(runtime, &content) {
                    return fail(runtime, e);
                }
            }

            for chunk in delta.tool_calls.unwrap_or_default() {
                let call = calls.entry(chunk.index).or_insert_with(|| AgentToolCall {
                    id: String::new(),
                    call_type: "function".to_string(),
                    function: AgentToolFunction {
                        name: String::new(),
                        arguments: String::new(),
                    },
                });
                if let Some(id) = chunk.id {
                    call.id = id;
                }
                if let Some(function) = chunk.function {
                    call.function.name.push_str(&function.name.unwrap_or_default());
                    call.function.arguments.push_str(&function.arguments.unwrap_or_default());
                }
            }
        }
    }

    let calls = calls.into_values().collect::<Vec<_>>();
    if calls.is_empty() {
        if let Err(e) = runtime.set_status(AgentStatus::Idle) {
            eprintln!("Agent 切换到空闲状态失败: {e}");
        }
        return;
    }

    if let Err(e) = runtime
        .history
        .update_last_message(|message| message.tool_calls = Some(calls.clone()))
    {
        return fail(runtime, format!("Agent 保存工具调用失败: {e}"));
    }
    enqueue_tool_calls(runtime, calls);
}

fn append_assistant(runtime: &AgentRuntime) -> Result<(), String> {
    runtime.history.append(AgentMessage {
        role: "assistant".to_string(),
        content: Some(String::new()),
        tool_name: None,
        tool_calls: None,
        tool_call_id: None,
    })
}

fn append_content(runtime: &AgentRuntime, delta: &str) -> Result<(), String> {
    runtime.history.update_last_message(|message| {
        message
            .content
            .get_or_insert_with(String::new)
            .push_str(delta);
    })
}

fn enqueue_tool_calls(runtime: &AgentRuntime, calls: Vec<AgentToolCall>) {
    for call in calls {
        if let Err(e) = runtime.tasks.push(AgentTask::ToolCall {
            tool_call_id: call.id,
            tool_name: call.function.name,
            payload: call.function.arguments,
        }) {
            return fail(runtime, format!("Agent 工具任务入队失败: {e}"));
        }
    }

    if let Err(e) = runtime.tasks.push(AgentTask::ContinueFromToolResults) {
        fail(runtime, format!("Agent 继续回复任务入队失败: {e}"));
    }
}

fn fail(runtime: &AgentRuntime, error: String) {
    let _ = runtime.set_status(AgentStatus::Idle);
    eprintln!("{error}");
}
