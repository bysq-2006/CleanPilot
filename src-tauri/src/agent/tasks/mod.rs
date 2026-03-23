use super::runtime::AgentRuntime;
use super::task_queue::AgentTask;
use crate::agent::history::{AgentMessage, AgentMessageRole};

pub mod chat;
pub mod reply;

pub async fn handle_task(runtime: &AgentRuntime, task: AgentTask) {

    match task {
        AgentTask::UserQuestion { content } => {
            chat::handle_user_question(runtime, content).await;
        }
        AgentTask::AssistantReply { content } => {
            reply::handle_assistant_reply(runtime, content);
        }
        AgentTask::ToolCall { tool_name, payload } => {
            println!("Agent 收到工具调用任务: {}, payload={}", tool_name, payload);

            let result = runtime.tools.call(&tool_name, &payload);

            let content = match result {
                Ok(output) => format!(
                    "工具调用结果\n工具名: {}\n参数: {}\n输出:\n{}",
                    tool_name, payload, output
                ),
                Err(error) => format!(
                    "工具调用失败\n工具名: {}\n参数: {}\n错误: {}",
                    tool_name, payload, error
                ),
            };

            if let Err(e) = runtime.history.append(AgentMessage {
                role: AgentMessageRole::Assistant,
                content,
            }) {
                eprintln!("Agent 写入工具结果失败: {}", e);
            }
        }
        AgentTask::ContinueReply => {
            chat::handle_continue_reply(runtime).await;
        }
    }
}
