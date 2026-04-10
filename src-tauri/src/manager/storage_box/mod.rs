pub mod file_ops;

use std::{fs, path::PathBuf, sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};

const STORAGE_BOX_DIR_NAME: &str = "storage_box";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBoxRecord {
    pub file_name: String,
    pub content: Value,
    pub saved_at: u64,
    pub task_type: String,
}

impl StorageBoxRecord {
    pub fn new(file_name: String, content: Value, task_type: String) -> Result<Self, String> {
        let file_name = file_name.trim().to_string();
        if file_name.is_empty() {
            return Err("文件名不能为空".to_string());
        }

        let task_type = task_type.trim().to_string();
        if task_type.is_empty() {
            return Err("任务类型不能为空".to_string());
        }

        let saved_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("系统时间异常: {}", e))?
            .as_secs();

        Ok(Self {
            file_name,
            content,
            saved_at,
            task_type,
        })
    }

}

#[derive(Debug, Clone, Default)]
pub struct StorageBoxManager {
    pub storage_box_dir_path: Arc<Mutex<PathBuf>>,
}

impl StorageBoxManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        Ok(Self {
            storage_box_dir_path: Arc::new(Mutex::new(
                app.path()
                    .app_data_dir()
                    .map_err(|e| format!("无法获取应用数据目录: {}", e))?
                    .join(STORAGE_BOX_DIR_NAME),
            )),
        })
    }

    pub fn read_record(&self, file_name: String) -> Result<StorageBoxRecord, String> {
        let file_path = self.resolve_file_path(&file_name)?;
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("读取 Storage Box 文件失败: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("反序列化 Storage Box 记录失败: {}", e))
    }

    pub fn save_new_record(
        &self,
        file_name: String,
        content: Value,
        task_type: String,
    ) -> Result<(), String> {
        let record = StorageBoxRecord::new(file_name, content, task_type)?;
        self.save_record(&record)
    }

    pub fn delete_record(&self, file_name: String) -> Result<(), String> {
        let file_path = self.resolve_file_path(&file_name)?;

        if !file_path.exists() {
            return Err(format!("Storage Box 文件不存在: {}", file_name));
        }

        fs::remove_file(&file_path)
            .map_err(|e| format!("删除 Storage Box 文件失败: {}", e))
    }

    fn get_storage_box_dir_path(&self) -> Result<PathBuf, String> {
        self.storage_box_dir_path
            .lock()
            .map_err(|e| format!("Storage Box 目录锁获取失败: {}", e))
            .map(|path| path.clone())
    }

    fn resolve_file_path(&self, file_name: &str) -> Result<PathBuf, String> {
        let file_name = file_name.trim();
        if file_name.is_empty() {
            return Err("文件名不能为空".to_string());
        }

        Ok(self.get_storage_box_dir_path()?.join(file_name))
    }

    pub fn operate_record_file<T>(
        &self,
        file_name: &str,
        operation: impl FnOnce(&std::path::Path) -> Result<T, String>,
    ) -> Result<T, String> {
        let file_path = self.resolve_file_path(file_name)?;
        operation(&file_path)
    }

    pub fn save_record(&self, record: &StorageBoxRecord) -> Result<(), String> {
        let storage_box_dir_path = self.get_storage_box_dir_path()?;

        fs::create_dir_all(&storage_box_dir_path)
            .map_err(|e| format!("创建 Storage Box 目录失败: {}", e))?;

        let file_path = self.resolve_file_path(&record.file_name)?;
        let content = serde_json::to_string_pretty(record)
            .map_err(|e| format!("序列化 Storage Box 记录失败: {}", e))?;

        fs::write(&file_path, content)
            .map_err(|e| format!("写入 Storage Box 文件失败: {}", e))?;

        Ok(())
    }

    /// 列出 Storage Box 中所有记录，按保存时间倒序排序。
    pub fn list_records(&self) -> Result<Vec<StorageBoxRecord>, String> {
        let storage_box_dir_path = self.get_storage_box_dir_path()?;

        if !storage_box_dir_path.exists() {
            return Ok(Vec::new());
        }

        let mut records = Vec::new();

        for entry in fs::read_dir(&storage_box_dir_path)
            .map_err(|e| format!("读取 Storage Box 目录失败: {}", e))?
        {
            let entry = entry.map_err(|e| format!("遍历 Storage Box 目录失败: {}", e))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("读取 Storage Box 文件失败: {}", e))?;
            let record: StorageBoxRecord = serde_json::from_str(&content)
                .map_err(|e| format!("反序列化 Storage Box 记录失败: {}", e))?;

            records.push(record);
        }

        records.sort_by(|a, b| {
            b.saved_at
                .cmp(&a.saved_at)
                .then_with(|| a.file_name.cmp(&b.file_name))
        });

        Ok(records)
    }
}
