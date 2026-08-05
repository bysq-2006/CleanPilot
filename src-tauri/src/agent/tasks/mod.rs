use super::runtime::AgentRuntime;
use super::tasks::queue::AgentTask;

pub mod chat;
pub mod queue;
pub mod tool_call;

pub async fn handle_task(runtime: &AgentRuntime, task: AgentTask) {
    if task.cancellation_token().is_cancelled() {
        return;
    }

    match task {
        AgentTask::UserQuestion { content, cancellation_token } =>
            chat::handle_user_question(runtime, content, cancellation_token).await,
        AgentTask::ContinueFromToolResults { cancellation_token } =>
            chat::handle_continue_reply(runtime, cancellation_token).await,
        AgentTask::ToolCall { tool_call_id, tool_name, payload, cancellation_token } =>
            tool_call::handle_tool_call(runtime, tool_call_id, tool_name, payload, cancellation_token).await,
    }
}
