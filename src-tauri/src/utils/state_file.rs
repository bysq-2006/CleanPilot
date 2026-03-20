use std::fs;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use tauri::{AppHandle, Manager};

fn state_file_path(app: &AppHandle, file_name: &str) -> Result<PathBuf, String> {
    if file_name.trim().is_empty() {
        return Err("文件名不能为空".to_string());
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    Ok(app_data_dir.join(file_name))
}

/// 从磁盘加载任意类型状态。
///
/// - `file_name`: 目标文件名（例如 `config.json`）
/// - 若文件不存在或内容为空，返回 `T::default()`
pub fn load_state_from_disk<T>(app: &AppHandle, file_name: &str) -> Result<T, String>
where
    T: DeserializeOwned + Default,
{
    let path = state_file_path(app, file_name)?;

    if !path.exists() {
        return Ok(T::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取 {} 失败: {} ({})", file_name, e, path.display()))?;

    if content.trim().is_empty() {
        return Ok(T::default());
    }

    let state: T =
        serde_json::from_str(&content).map_err(|e| format!("解析 {} 失败: {}", file_name, e))?;

    Ok(state)
}

/// 将任意类型状态保存到磁盘。
///
/// - `file_name`: 目标文件名（例如 `config.json`）
pub fn save_state_to_disk<T>(app: &AppHandle, file_name: &str, state: &T) -> Result<(), String>
where
    T: serde::Serialize,
{
    let path = state_file_path(app, file_name)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("创建应用数据目录失败: {} ({})", e, parent.display()))?;
    }

    let json = serde_json::to_string_pretty(state)
        .map_err(|e| format!("序列化配置失败: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("写入 {} 失败: {} ({})", file_name, e, path.display()))
}
