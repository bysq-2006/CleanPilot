use tauri::{AppHandle, Manager};

use crate::models::appstore::AppStore;

#[tauri::command]
pub fn get_config(
    app: AppHandle,
    section: String,
) -> Result<serde_json::Value, String> {
    let store = app.state::<AppStore>();
    let config = store
        .config
        .lock()
        .map_err(|e| format!("配置锁获取失败: {}", e))?;

    config.get_section(&section)
}

#[tauri::command]
pub fn save_config(
    app: AppHandle,
    section: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let mut config = store
        .config
        .lock()
        .map_err(|e| format!("配置锁获取失败: {}", e))?;

    config.save_section(&section, value)
}

