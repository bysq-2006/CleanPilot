use super::runtime::AgentRuntime;
use super::task_queue::AgentTask;

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
        }
        AgentTask::ContinueReply => {
            chat::handle_continue_reply(runtime).await;
        }
    }
}
