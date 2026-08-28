pub mod file_ops;

use std::{fs, path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

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

#[derive(Debug, Clone)]
pub struct StorageBoxManager {
    pub path: Arc<PathBuf>,
}

impl StorageBoxManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        Ok(Self {
            path: Arc::new(
                app.path()
                    .app_data_dir()
                    .map_err(|e| format!("无法获取应用数据目录: {}", e))?
                    .join(STORAGE_BOX_DIR_NAME),
            ),
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
        let file_name = sanitize_record_file_name(&file_name)?;
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

    fn resolve_file_path(&self, file_name: &str) -> Result<PathBuf, String> {
        let file_name = file_name.trim();
        if file_name.is_empty() {
            return Err("文件名不能为空".to_string());
        }
        if PathBuf::from(file_name).components().count() != 1 {
            return Err("Storage Box 记录名不能包含路径".to_string());
        }

        Ok(self.path.join(file_name))
    }

    pub fn save_record(&self, record: &StorageBoxRecord) -> Result<(), String> {
        fs::create_dir_all(&*self.path)
            .map_err(|e| format!("创建 Storage Box 目录失败: {}", e))?;

        let file_path = self.resolve_file_path(&record.file_name)?;
        let content = serde_json::to_string_pretty(record)
            .map_err(|e| format!("序列化 Storage Box 记录失败: {}", e))?;

        fs::write(&file_path, content)
            .map_err(|e| format!("写入 Storage Box 文件失败: {}", e))
    }

    /// 列出 Storage Box 中所有记录，按保存时间倒序排序。
    pub fn list_records(&self) -> Result<Vec<StorageBoxRecord>, String> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let mut records = Vec::new();

        for entry in fs::read_dir(&*self.path)
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

fn is_illegal_file_name_char(c: char) -> bool {
    matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control()
}

/// 把 LLM 给的 title 收成可落盘的单层文件名，避免 `/`、`:` 等被当成路径。
fn sanitize_record_file_name(file_name: &str) -> Result<String, String> {
    let file_name = file_name.trim();
    if file_name.is_empty() {
        return Err("文件名不能为空".to_string());
    }

    let mut sanitized = String::with_capacity(file_name.len());
    let mut last_was_underscore = false;
    for c in file_name.chars() {
        if is_illegal_file_name_char(c) {
            if !last_was_underscore {
                sanitized.push('_');
                last_was_underscore = true;
            }
        } else {
            sanitized.push(c);
            last_was_underscore = false;
        }
    }

    let mut sanitized = sanitized
        .trim_matches(|c: char| c == '_' || c == '.' || c.is_whitespace())
        .to_string();

    if sanitized.is_empty() {
        sanitized = "任务清单.json".to_string();
    }

    const MAX_CHARS: usize = 180;
    if sanitized.chars().count() > MAX_CHARS {
        let keep_json = file_name.ends_with(".json");
        let take = if keep_json { MAX_CHARS.saturating_sub(5) } else { MAX_CHARS };
        let stem: String = sanitized
            .strip_suffix(".json")
            .unwrap_or(&sanitized)
            .chars()
            .take(take)
            .collect();
        let stem = stem.trim_end_matches(|c: char| c == '_' || c == '.' || c.is_whitespace());
        sanitized = if keep_json {
            format!("{}.json", stem)
        } else {
            stem.to_string()
        };
    }

    if sanitized.is_empty() || PathBuf::from(&sanitized).components().count() != 1 {
        return Err("Storage Box 记录名不能包含路径".to_string());
    }

    Ok(sanitized)
}

#[cfg(test)]
mod tests {
    use super::sanitize_record_file_name;
    use std::path::PathBuf;

    #[test]
    fn sanitizes_slashes_in_title() {
        let name = sanitize_record_file_name("C盘激进清理候选（缓存/临时/可重装项）-1787896528.json")
            .expect("sanitize");
        assert_eq!(name, "C盘激进清理候选（缓存_临时_可重装项）-1787896528.json");
        assert_eq!(PathBuf::from(&name).components().count(), 1);
    }

    #[test]
    fn sanitizes_windows_drive_prefix() {
        let name = sanitize_record_file_name("C:清理-1.json").expect("sanitize");
        assert_eq!(name, "C_清理-1.json");
        assert_eq!(PathBuf::from(&name).components().count(), 1);
    }

    #[test]
    fn rejects_empty_name() {
        assert!(sanitize_record_file_name("   ").is_err());
    }
}
