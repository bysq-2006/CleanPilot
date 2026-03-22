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
    store.agent.history.get_from(start_index)
}
