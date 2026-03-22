use crate::agent::history::{AgentMessage, AgentMessageRole};
use crate::agent::runtime::AgentRuntime;

pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
    if let Err(e) = runtime.history.append(AgentMessage {
        role: AgentMessageRole::User,
        content: content.clone(),
    }) {
        eprintln!("Agent 写入历史记录失败: {}", e);
    }

    match runtime.llm.chat(content.clone()).await {
        Ok(reply) => {
            if let Err(e) = runtime.history.append(AgentMessage {
                role: AgentMessageRole::Assistant,
                content: reply.clone(),
            }) {
                eprintln!("Agent 写入回复历史失败: {}", e);
            }

            println!("Agent 回复用户问题: {}", reply);
        }
        Err(e) => {
            let error_message = format!("LLM 调用失败: {}", e);

            if let Err(history_error) = runtime.history.append(AgentMessage {
                role: AgentMessageRole::Assistant,
                content: error_message.clone(),
            }) {
                eprintln!("Agent 写入错误历史失败: {}", history_error);
            }

            eprintln!("{}", error_message);
        }
    }

    println!("Agent 收到用户问题任务: {}", content);
}
