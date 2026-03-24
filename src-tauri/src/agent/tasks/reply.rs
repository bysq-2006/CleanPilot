use crate::agent::context::history::AgentMessage;
use crate::agent::runtime::AgentRuntime;

pub fn handle_assistant_reply(runtime: &AgentRuntime, content: String) {
    if let Err(e) = runtime.history.append(AgentMessage {
        role: "assistant".to_string(),
        content: Some(content.clone()),
        tool_calls: None,
        tool_call_id: None,
    }) {
        eprintln!("Agent 写入 Assistant 历史失败: {}", e);
        return;
    }
}
