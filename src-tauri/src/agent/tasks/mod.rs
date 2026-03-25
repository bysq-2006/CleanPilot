use super::runtime::AgentRuntime;
use super::tasks::queue::AgentTask;

pub mod chat;
pub mod queue;
pub mod tool_call;

pub async fn handle_task(runtime: &AgentRuntime, task: AgentTask) {

    match task {
        AgentTask::UserQuestion { content } => {
            chat::handle_user_question(runtime, content).await;
        }
        AgentTask::ToolCall { tool_call_id, tool_name, payload } => {
            tool_call::handle_tool_call(runtime, tool_call_id, tool_name, payload).await;
        }
    }
}
