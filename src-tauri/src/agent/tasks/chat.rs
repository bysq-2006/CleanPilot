use crate::agent::context::history::{AgentMessage, AgentToolCall};
use crate::agent::runtime::AgentRuntime;
use crate::agent::tasks::queue::AgentTask;
use futures_util::StreamExt;

/// 处理用户问题任务：先写入 user 消息，再把 LLM 原始输出交给 agent 侧解析器处理。
pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
    println!("Agent 收到用户问题任务: {}", content);

    if let Err(e) = runtime.history.append(AgentMessage {
        role: "user".to_string(),
        content: Some(content),
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
            let mut phase = "search_content";
            let mut content = String::new();
            let mut tool_calls_raw = String::new();
            let mut tool_calls_depth = 0_i32;

            if let Err(e) = runtime.history.append(AgentMessage {
                role: "assistant".to_string(),
                content: Some(String::new()),
                tool_calls: None,
                tool_call_id: None,
            }) {
                eprintln!("Agent 创建空 Assistant 消息失败: {}", e);
                return;
            }

            let sync_last_message = |runtime: &AgentRuntime, content: &str| {
                if let Err(e) = runtime.history.update_last_message(|message| {
                    message.content = Some(content.to_string());
                }) {
                    eprintln!("Agent 更新最后一条 Assistant 消息失败: {}", e);
                }
            };

            let read_content = |raw: &str| -> String {
                let key = "\"content\":\"";
                let Some(start) = raw.find(key) else {
                    return String::new();
                };

                let mut escaped = false;
                let mut reading_unicode = false;
                let mut unicode = String::new();
                let mut result = String::new();

                for ch in raw[start + key.len()..].chars() {
                    if reading_unicode {
                        unicode.push(ch);
                        if unicode.len() == 4 {
                            if let Ok(code) = u32::from_str_radix(&unicode, 16) {
                                if let Some(decoded) = char::from_u32(code) {
                                    result.push(decoded);
                                }
                            }
                            unicode.clear();
                            reading_unicode = false;
                            escaped = false;
                        }
                        continue;
                    }

                    if escaped {
                        match ch {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            '/' => result.push('/'),
                            'b' => result.push('\u{0008}'),
                            'f' => result.push('\u{000C}'),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'u' => {
                                reading_unicode = true;
                                unicode.clear();
                            }
                            other => result.push(other),
                        }

                        if !reading_unicode {
                            escaped = false;
                        }
                        continue;
                    }

                    match ch {
                        '\\' => escaped = true,
                        '"' => break,
                        other => result.push(other),
                    }
                }

                result
            };

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(response) => {
                        for choice in response.choices {
                            if let Some(delta) = choice.delta.content {
                                raw_reply.push_str(&delta);

                                if phase == "search_content" && raw_reply.contains("\"content\":\"") {
                                    phase = "reading_content";
                                }

                                if phase == "reading_content" {
                                    content = read_content(&raw_reply);
                                    sync_last_message(runtime, &content);

                                    if raw_reply.contains("\"tool_calls\":") {
                                        phase = "waiting_tool_calls";
                                    }
                                }

                                if phase == "waiting_tool_calls" {
                                    if let Some(index) = raw_reply.find("\"tool_calls\":") {
                                        let suffix = &raw_reply[index + "\"tool_calls\":".len()..];
                                        tool_calls_raw.clear();
                                        tool_calls_depth = 0;

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
                                    }
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
                match serde_json::from_str::<Vec<AgentToolCall>>(&tool_calls_raw) {
                    Ok(tool_calls) => {
                        for tool_call in tool_calls {
                            if let Err(e) = runtime.tasks.push(AgentTask::ToolCall {
                                tool_call_id: tool_call.id,
                                tool_name: tool_call.function.name,
                                payload: tool_call.function.arguments,
                            }) {
                                eprintln!("{}: {}", enqueue_error_prefix, e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("tool_calls 解析失败: {}\n原始输出: {}", e, tool_calls_raw);
                    }
                }
            }

            println!("{}: {}", success_log, raw_reply);
        }
        Err(e) => eprintln!("LLM 调用失败: {}", e),
    }
}
