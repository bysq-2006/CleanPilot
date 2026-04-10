use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::manager::storage_box::StorageBoxRecord;
use crate::manager::storage_box::file_ops;
use crate::models::appstore::AppStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskCleanupItem {
    pub path: String,
    pub purpose: String,
}

#[tauri::command]
pub fn list_storage_box_record_metas(app: AppHandle) -> Result<Vec<StorageBoxRecord>, String> {
    let store = app.state::<AppStore>();
    store.manager.storage_box.list_records()
}

#[tauri::command]
pub fn get_disk_cleanup_items(app: AppHandle, path: String) -> Result<Vec<DiskCleanupItem>, String> {
    let store = app.state::<AppStore>();
    let record = store.manager.storage_box.read_record(path)?;

    serde_json::from_value(record.content)
        .map_err(|e| format!("解析 disk_cleanup 条目失败: {}", e))
}

#[tauri::command]
pub fn save_disk_cleanup_items(app: AppHandle, path: String, items: Vec<DiskCleanupItem>) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let mut record = store.manager.storage_box.read_record(path)?;

    record.content = serde_json::to_value(items)
        .map_err(|e| format!("序列化 disk_cleanup 条目失败: {}", e))?;

    store.manager.storage_box.save_record(&record)
}

#[tauri::command]
pub fn delete_storage_box_record(app: AppHandle, path: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    store.manager.storage_box.delete_record(path)
}

#[tauri::command]
pub fn reveal_storage_box_path(app: AppHandle, path: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    store
        .manager
        .storage_box
        .operate_record_file(&path, file_ops::reveal_in_file_manager)
}

#[tauri::command]
pub fn trash_storage_box_path(app: AppHandle, path: String) -> Result<(), String> {
    let store = app.state::<AppStore>();
    store
        .manager
        .storage_box
        .operate_record_file(&path, file_ops::move_to_trash)
}
