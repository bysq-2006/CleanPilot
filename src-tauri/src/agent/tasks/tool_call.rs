use crate::agent::context::history::AgentMessage;
use crate::agent::runtime::AgentRuntime;

pub async fn handle_tool_call(
    runtime: &AgentRuntime,
    tool_call_id: String,
    tool_name: String,
    payload: String,
) {
    println!("Agent 收到工具调用任务: {}, payload={}", tool_name, payload);

    let result = runtime.tools.call(runtime, &tool_name, &payload).await;

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
        role: "tool".to_string(),
        content: Some(content),
        tool_calls: None,
        tool_call_id: Some(tool_call_id),
    }) {
        eprintln!("Agent 写入工具结果失败: {}", e);
        return;
    }
}
