use std::collections::BTreeMap;
use std::sync::Arc;

use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::agent::context::history::{AgentMessage, AgentToolCall, AgentToolFunction};
use crate::agent::runtime::AgentRuntime;
use crate::agent::tasks::queue::AgentTask;

pub async fn handle_user_question(runtime: &AgentRuntime, content: String, cancellation_token: Arc<CancellationToken>) {
    match runtime.history.append_if_active(&cancellation_token, AgentMessage {
        role: "user".to_string(),
        content: Some(content),
        tool_name: None,
        tool_calls: None,
        tool_call_id: None,
    }) {
        Ok(true) => request_and_enqueue_tasks(runtime, cancellation_token).await,
        Ok(false) => {}
        Err(e) => fail(runtime, &cancellation_token, format!("Agent 写入用户消息失败: {e}")),
    }
}

pub async fn handle_continue_reply(runtime: &AgentRuntime, cancellation_token: Arc<CancellationToken>) {
    request_and_enqueue_tasks(runtime, cancellation_token).await;
}

async fn request_and_enqueue_tasks(runtime: &AgentRuntime, cancellation_token: Arc<CancellationToken>) {
    if cancellation_token.is_cancelled() {
        return;
    }
    let tools = match runtime.tools.lock() {
        Ok(tools) => tools.api_tools(),
        Err(e) => return fail(runtime, &cancellation_token, format!("Agent 工具锁获取失败: {e}")),
    };
    let stream_result = tokio::select! {
        _ = cancellation_token.cancelled() => return,
        result = runtime.llm.chat_stream(&runtime.history, tools) => result,
    };
    let mut stream = match stream_result {
        Ok(stream) => stream,
        Err(e) => return fail(runtime, &cancellation_token, format!("LLM 调用失败: {e}")),
    };

    match append_assistant(runtime, &cancellation_token) {
        Ok(true) => {}
        Ok(false) => return,
        Err(e) => return fail(runtime, &cancellation_token, e),
    }

    let mut calls = BTreeMap::<i32, AgentToolCall>::new();
    loop {
        let chunk = tokio::select! {
            _ = cancellation_token.cancelled() => return,
            chunk = stream.next() => match chunk {
                Some(chunk) => chunk,
                None => break,
            },
        };
        let response = match chunk {
            Ok(response) => response,
            Err(e) => return fail(runtime, &cancellation_token, format!("LLM 流式响应失败: {e}")),
        };

        for choice in response.choices {
            if cancellation_token.is_cancelled() { return; }
            let delta = choice.delta;
            if let Some(content) = delta.content.or(delta.refusal) {
                if let Err(e) = append_content(runtime, &cancellation_token, &content) {
                    return fail(runtime, &cancellation_token, e);
                }
            }

            for chunk in delta.tool_calls.unwrap_or_default() {
                if cancellation_token.is_cancelled() { return; }
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

    if cancellation_token.is_cancelled() { return; }
    let calls = calls.into_values().collect::<Vec<_>>();
    if calls.is_empty() {
        if let Err(e) = runtime.finish_if_current(&cancellation_token) {
            log::error!("Agent 切换到空闲状态失败: {e}");
        }
        return;
    }

    if let Err(e) = runtime
        .history
        .update_last_message(|message| {
            if !cancellation_token.is_cancelled() {
                message.tool_calls = Some(calls.clone());
            }
        })
    {
        return fail(runtime, &cancellation_token, format!("Agent 保存工具调用失败: {e}"));
    }
    enqueue_tool_calls(runtime, calls, cancellation_token);
}

fn append_assistant(runtime: &AgentRuntime, cancellation_token: &CancellationToken) -> Result<bool, String> {
    runtime.history.append_if_active(cancellation_token, AgentMessage {
        role: "assistant".to_string(),
        content: Some(String::new()),
        tool_name: None,
        tool_calls: None,
        tool_call_id: None,
    })
}

fn append_content(runtime: &AgentRuntime, cancellation_token: &CancellationToken, delta: &str) -> Result<(), String> {
    runtime.history.update_last_message(|message| {
        if !cancellation_token.is_cancelled() {
            message.content.get_or_insert_with(String::new).push_str(delta);
        }
    })
}

fn enqueue_tool_calls(runtime: &AgentRuntime, calls: Vec<AgentToolCall>, cancellation_token: Arc<CancellationToken>) {
    for call in calls {
        if cancellation_token.is_cancelled() { return; }
        if let Err(e) = runtime.tasks.push(AgentTask::ToolCall {
            tool_call_id: call.id,
            tool_name: call.function.name,
            payload: call.function.arguments,
            cancellation_token: cancellation_token.clone(),
        }) {
            return fail(runtime, &cancellation_token, format!("Agent 工具任务入队失败: {e}"));
        }
        if cancellation_token.is_cancelled() {
            let _ = runtime.tasks.clear();
            return;
        }
    }

    if let Err(e) = runtime.tasks.push(AgentTask::ContinueFromToolResults {
        cancellation_token: cancellation_token.clone(),
    }) {
        fail(runtime, &cancellation_token, format!("Agent 继续回复任务入队失败: {e}"));
    } else if cancellation_token.is_cancelled() {
        let _ = runtime.tasks.clear();
    }
}

fn fail(runtime: &AgentRuntime, cancellation_token: &Arc<CancellationToken>, error: String) {
    if cancellation_token.is_cancelled() { return; }
    let _ = runtime.finish_if_current(cancellation_token);
    log::error!("{error}");
}
