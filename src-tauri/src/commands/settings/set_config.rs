use tauri::{AppHandle, Manager};

use crate::models::appstore::AppStore;

#[tauri::command]
pub fn get_config(
    app: AppHandle,
    section: String,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    let store = app.state::<AppStore>();
    let config = store
        .config
        .lock()
        .map_err(|e| format!("配置锁获取失败: {}", e))?;

    Ok(config.get(&section, &key))
}

#[tauri::command]
pub fn save_config(
    app: AppHandle,
    section: String,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let mut config = store
        .config
        .lock()
        .map_err(|e| format!("配置锁获取失败: {}", e))?;

    config.save(section, key, value)
}

