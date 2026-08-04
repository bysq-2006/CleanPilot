use serde::{Deserialize, Serialize};
use std::path::Path;
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
pub fn reveal_storage_box_path(app: AppHandle, record_path: String, path: String) -> Result<(), String> {
    let items = get_disk_cleanup_items(app.clone(), record_path)?;
    if !items.iter().any(|item| item.path == path) {
        return Err("目标路径不属于当前清理任务".to_string());
    }

    file_ops::reveal_in_file_manager(Path::new(&path))
}

#[tauri::command]
pub fn recycle_disk_cleanup_item(
    app: AppHandle,
    record_path: String,
    path: String,
) -> Result<(), String> {
    let store = app.state::<AppStore>();
    let mut record = store.manager.storage_box.read_record(record_path)?;
    if record.task_type != "disk_cleanup" {
        return Err("当前任务不是磁盘清理任务".to_string());
    }

    let mut items: Vec<DiskCleanupItem> = serde_json::from_value(record.content.clone())
        .map_err(|e| format!("解析 disk_cleanup 条目失败: {}", e))?;
    if !items.iter().any(|item| item.path == path) {
        return Err("目标路径不属于当前清理任务".to_string());
    }

    file_ops::move_to_recycle_bin(Path::new(&path))?;
    items.retain(|item| item.path != path);
    record.content = serde_json::to_value(items)
        .map_err(|e| format!("序列化 disk_cleanup 条目失败: {}", e))?;

    store.manager.storage_box.save_record(&record)
}
