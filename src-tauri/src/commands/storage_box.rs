use tauri::{AppHandle, Manager};

use crate::manager::storage_box::StorageBoxRecordMeta;
use crate::manager::storage_box::file_ops;
use crate::models::appstore::AppStore;

#[tauri::command]
pub fn list_storage_box_record_metas(app: AppHandle) -> Result<Vec<StorageBoxRecordMeta>, String> {
    let store = app.state::<AppStore>();
    store.manager.storage_box.list_record_metas()
}

#[tauri::command]
pub fn reveal_storage_box_path(path: String) -> Result<(), String> {
    file_ops::reveal_in_file_manager(std::path::Path::new(&path))
}

#[tauri::command]
pub fn trash_storage_box_path(path: String) -> Result<(), String> {
    file_ops::move_to_trash(std::path::Path::new(&path))
}
