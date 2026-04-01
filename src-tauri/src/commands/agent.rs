use tauri::{AppHandle, Manager};

use crate::agent::context::history::AgentMessage;
use crate::agent::tasks::queue::AgentTask;
use crate::models::appstore::AppStore;

#[tauri::command]
/// 接收前端输入并将其作为用户问题压入 Agent 任务队列。
pub fn chat(app: AppHandle, content: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let agent = store
        .agent
        .lock()
        .map_err(|e| format!("Agent 锁获取失败: {}", e))?;

    let agent = agent
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?;

    agent.tasks.push(AgentTask::UserQuestion { content })?;

    Ok(())
}

#[tauri::command]
/// 按起始索引返回当前会话新增的历史消息，供前端增量同步聊天内容。
pub fn get_history(app: AppHandle, start_index: usize) -> Result<Vec<AgentMessage>, String> {
    let store = app.state::<AppStore>();
    let agent = store
        .agent
        .lock()
        .map_err(|e| format!("Agent 锁获取失败: {}", e))?;
    let history = agent
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?
        .history
        .inner
        .lock()
        .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

    if start_index >= history.len() {
        return Ok(Vec::new());
    }

    Ok(history[start_index..].to_vec())
}

#[tauri::command]
/// 将当前 Agent 最终发送给 LLM 的完整输入打印到控制台，便于调试检查。
pub fn debug_print_history(app: AppHandle) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let agent = store
        .agent
        .lock()
        .map_err(|e| format!("Agent 锁获取失败: {}", e))?;
    let history = agent
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?
        .history
        .build_llm_input()?;

    println!("Agent 调试输出完整 history: {}", history);

    Ok(())
}
