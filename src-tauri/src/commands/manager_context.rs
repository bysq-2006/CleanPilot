use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::manager::agent_scene::AgentScene;
use crate::manager::history::HistoryRecordSummary;
use crate::models::appstore::AppStore;

#[tauri::command]
/// 读取并返回本地已保存的全部历史会话摘要列表，供聊天记录页展示。
pub fn list_history_records(app: AppHandle) -> Result<Vec<HistoryRecordSummary>, String> {
    let store = app.state::<AppStore>();
    store.manager.history.list_history_records()
}

#[tauri::command]
/// 获取当前 Agent 场景的名称字符串。
pub fn get_current_scene(app: AppHandle) -> Result<String, String> {
    let store = app.state::<AppStore>();
    Ok(store.manager.agent_scene.get_current_scene()?.as_str().to_string())
}

#[tauri::command]
/// 切换当前 Agent 场景并应用到 Agent 实例。
pub fn switch_current_scene(app: AppHandle, scene: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let agent = store
        .agent
        .lock()
        .map_err(|e| format!("Agent 锁获取失败: {}", e))?;
    let agent = agent
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?;

    let scene = AgentScene::from_str(&scene)?;
    store.manager.agent_scene.switch_scene(scene, agent)
}

#[tauri::command]
/// 创建一个新的会话 UUID 并写入当前历史上下文，同时切换到磁盘清理场景。
pub fn create_history_context(app: AppHandle) -> Result<String, String> {
    let store = app.state::<AppStore>();
    let context_id = Uuid::new_v4().to_string();
    let agent = store
        .agent
        .lock()
        .map_err(|e| format!("Agent 锁获取失败: {}", e))?;
    let agent = agent
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?;

    store.manager.agent_scene.switch_scene(AgentScene::DiskCleanup, agent)?;
    store.manager.history.set_current_context_id(context_id.clone())?;

    Ok(context_id)
}

#[tauri::command]
/// 根据给定 context_id 恢复历史记录和场景，并将其应用到当前 Agent。
pub fn restore_history_context(app: AppHandle, context_id: String) -> Result<HistoryRecordSummary, String> {
    let store = app.state::<AppStore>();
    let agent = store
        .agent
        .lock()
        .map_err(|e| format!("Agent 锁获取失败: {}", e))?;
    let agent = agent
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?;

    store.manager.history.set_current_context_id(context_id)?;

    let record = store.manager.history.load_context_items()?;
    let scene = AgentScene::from_str(&record.scene)?;

    store.manager.agent_scene.switch_scene(scene, agent)?;

    let mut history = agent
        .history
        .inner
        .lock()
        .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;
    *history = record.items.clone();

    Ok(record)
}

#[tauri::command]
pub fn delete_history_context(app: AppHandle, context_id: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    store.manager.history.delete_context(context_id)
}
