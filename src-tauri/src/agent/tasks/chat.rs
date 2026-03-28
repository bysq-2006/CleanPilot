use crate::agent::context::history::{AgentMessage, AgentToolCall};
use crate::agent::runtime::AgentRuntime;
use crate::agent::tasks::queue::AgentTask;
use crate::utils::text_decode::decode_escaped_text;
use futures_util::StreamExt;

/// 处理用户问题任务：先写入 user 消息，再把 LLM 原始输出交给 agent 侧解析器处理。
pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
    println!("Agent 收到用户问题任务: {}", content);

    if let Err(e) = runtime.history.append(AgentMessage {
        role: "user".to_string(),
        content: Some(content),
        tool_name: None,
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

/// 工具执行后继续请求 LLM。
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
            let mut raw_reply = String::new();
            let mut tool_calls_raw = String::new();
            let mut tool_calls_depth = 0_i32;

            if let Err(e) = append_empty_assistant_message(runtime) {
                eprintln!("Agent 创建空 Assistant 消息失败: {}", e);
                return;
            }

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(response) => {
                        for choice in response.choices {
                            if let Some(delta) = choice.delta.content {
                                raw_reply.push_str(&delta);

                                if raw_reply.contains("\"content\"") {
                                    sync_content_message(runtime, &raw_reply);
                                }

                                if raw_reply.contains("\"tool_calls\":") {
                                    (tool_calls_raw, tool_calls_depth) =
                                        collect_tool_calls(&raw_reply);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("LLM 流式分片处理失败: {}", e);
                        return;
                    }
                }
            }

            if !tool_calls_raw.is_empty() && tool_calls_depth == 0 {
                process_tool_calls(runtime, &tool_calls_raw, enqueue_error_prefix);
            }

            println!("{}: {}", success_log, raw_reply);
        }
        Err(e) => eprintln!("LLM 调用失败: {}", e),
    }
}

fn append_empty_assistant_message(runtime: &AgentRuntime) -> Result<(), String> {
    runtime.history.append(AgentMessage {
        role: "assistant".to_string(),
        content: Some(String::new()),
        tool_name: None,
        tool_calls: None,
        tool_call_id: None,
    })
}

fn sync_last_message(runtime: &AgentRuntime, content: &str) {
    if let Err(e) = runtime.history.update_last_message(|message| {
        message.content = Some(content.to_string());
    }) {
        eprintln!("Agent 更新最后一条 Assistant 消息失败: {}", e);
    }
}

fn sync_content_message(
    runtime: &AgentRuntime,
    raw_reply: &str,
) {
    let Some(content_key_index) = raw_reply.find("\"content\"") else {
        return;
    };

    let value_area = &raw_reply[content_key_index + "\"content\"".len()..];
    let value_offset = raw_reply.len() - value_area.len();

    let Some(start_quote_offset) = value_area.find('"') else {
        return;
    };
    let start = value_offset + start_quote_offset + 1;

    let mut escaped = false;
    let mut end = raw_reply.len();
    let content_slice = &raw_reply[start..];

    for (offset, ch) in content_slice.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => {
                end = start + offset;
                break;
            }
            _ => {}
        }
    }

    let decoded = decode_escaped_text(&raw_reply[start..end]);
    sync_last_message(runtime, &decoded);
}

fn collect_tool_calls(raw_reply: &str) -> (String, i32) {
    let Some(index) = raw_reply.find("\"tool_calls\":") else {
        return (String::new(), 0);
    };

    let suffix = &raw_reply[index + "\"tool_calls\":".len()..];
    let mut tool_calls_raw = String::new();
    let mut tool_calls_depth = 0;
    let mut started = false;

    for ch in suffix.chars() {
        if !started {
            if ch == '[' {
                started = true;
                tool_calls_depth = 1;
                tool_calls_raw.push(ch);
            }
            continue;
        }

        tool_calls_raw.push(ch);
        match ch {
            '[' => tool_calls_depth += 1,
            ']' => {
                tool_calls_depth -= 1;
                if tool_calls_depth == 0 {
                    break;
                }
            }
            _ => {}
        }
    }

    (tool_calls_raw, tool_calls_depth)
}

fn process_tool_calls(runtime: &AgentRuntime, tool_calls_raw: &str, enqueue_error_prefix: &str) {
    match serde_json::from_str::<Vec<AgentToolCall>>(tool_calls_raw) {
        Ok(tool_calls) => {
            if let Err(e) = runtime.history.update_last_message(|message| {
                message.tool_calls = Some(tool_calls.clone());
            }) {
                eprintln!("Agent 更新最后一条 Assistant 的 tool_calls 失败: {}", e);
            }

            enqueue_tool_calls(runtime, tool_calls, enqueue_error_prefix);
        }
        Err(e) => {
            eprintln!("tool_calls 解析失败: {}\n原始输出: {}", e, tool_calls_raw);
        }
    }
}

fn enqueue_tool_calls(
    runtime: &AgentRuntime,
    tool_calls: Vec<AgentToolCall>,
    enqueue_error_prefix: &str,
) {
    for tool_call in tool_calls {
        if let Err(e) = runtime.tasks.push(AgentTask::ToolCall {
            tool_call_id: tool_call.id,
            tool_name: tool_call.function.name,
            payload: tool_call.function.arguments,
        }) {
            eprintln!("{}: {}", enqueue_error_prefix, e);
            return;
        }
    }

    if let Err(e) = runtime.tasks.push(AgentTask::ContinueFromToolResults) {
                eprintln!("{}: {}", enqueue_error_prefix, e);
    }
}
