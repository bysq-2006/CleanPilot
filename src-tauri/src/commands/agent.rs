use tauri::{AppHandle, Manager};

use crate::agent::history::AgentMessage;
use crate::agent::task_queue::AgentTask;
use crate::models::appstore::AppStore;

#[tauri::command]
pub fn chat(app: AppHandle, content: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    store.agent.tasks.push(AgentTask::UserQuestion { content })
}

#[tauri::command]
pub fn get_history(app: AppHandle, start_index: usize) -> Result<Vec<AgentMessage>, String> {
    let store = app.state::<AppStore>();
    let history = store
        .agent
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
pub fn debug_print_history(app: AppHandle) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let history = store.agent.history.build_llm_input()?;

    println!("Agent 调试输出完整 history: {}", history);

    Ok(())
}
