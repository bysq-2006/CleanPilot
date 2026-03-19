use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::models::config::Config;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct PersistedState {
    pub config: Config,
}

fn state_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?;

    Ok(app_data_dir.join("state.json"))
}

pub fn load_config_from_disk(app: &AppHandle) -> Result<Config, String> {
    let path = state_file_path(app)?;

    if !path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取 state.json 失败: {} ({})", e, path.display()))?;

    if content.trim().is_empty() {
        return Ok(Config::default());
    }

    let persisted: PersistedState =
        serde_json::from_str(&content).map_err(|e| format!("解析 state.json 失败: {}", e))?;

    Ok(persisted.config)
}

pub fn save_config_to_disk(app: &AppHandle, config: &Config) -> Result<(), String> {
    let path = state_file_path(app)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("创建应用数据目录失败: {} ({})", e, parent.display()))?;
    }

    let persisted = PersistedState {
        config: config.clone(),
    };

    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|e| format!("序列化配置失败: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("写入 state.json 失败: {} ({})", e, path.display()))
}

