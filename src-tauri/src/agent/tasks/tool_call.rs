use crate::agent::context::history::AgentMessage;
use crate::agent::runtime::AgentRuntime;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

pub async fn handle_tool_call(
    runtime: &AgentRuntime,
    tool_call_id: String,
    tool_name: String,
    payload: String,
    cancellation_token: Arc<CancellationToken>,
) {
    if cancellation_token.is_cancelled() {
        return;
    }
    log::debug!("Agent 收到工具调用任务: {}", tool_name);

    let tools = match runtime.tools.lock() {
        Ok(tools) => tools.clone(),
        Err(error) => {
            log::error!("Agent 工具锁获取失败: {}", error);
            return;
        }
    };

    let result = tools.call(runtime, &tool_name, &payload, &cancellation_token).await;
    if cancellation_token.is_cancelled() {
        return;
    }

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

    if let Err(e) = runtime.history.append_if_active(&cancellation_token, AgentMessage {
        role: "tool".to_string(),
        content: Some(content),
        tool_name: Some(tool_name),
        tool_calls: None,
        tool_call_id: Some(tool_call_id),
    }) {
        log::error!("Agent 写入工具结果失败: {}", e);
        return;
    }
}
