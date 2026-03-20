use std::collections::HashMap;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::utils::state_file::{load_state_from_disk, save_state_to_disk};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub config_file_path: PathBuf,
    /// 双层 HashMap: section -> (key -> value)
    #[serde(default)]
    pub data: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl Config {
    /// 设置配置文件路径并加载配置
    pub fn init(&mut self, app: &AppHandle) -> Result<(), String> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

        self.config_file_path = app_data_dir.join("config.json");

        self.load_and_sync()
    }

    /// 读取配置并同步到内存
    pub fn load_and_sync(&mut self) -> Result<(), String> {
        let dir_path = self.config_dir()?;
        let disk_config: Config = load_state_from_disk(&dir_path, "config.json")?;
        self.data = disk_config.data;
        Ok(())
    }

    /// 获取配置文件所在目录路径
    fn config_dir(&self) -> Result<String, String> {
        if self.config_file_path.as_os_str().is_empty() {
            return Err("配置路径未初始化，请先调用 init".to_string());
        }

        self.config_file_path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .ok_or_else(|| format!("配置文件路径无效: {}", self.config_file_path.display()))
    }

    /// 获取配置项的值
    /// - `key`: 配置项的键
    /// - 返回值：配置项的值，如果不存在则返回 `None`
    pub fn get(&self, section: &str, key: &str) -> Option<serde_json::Value> {
        self.data.get(section).and_then(|m| m.get(key)).cloned()
    }

    /// 保存单个配置项，并立即写入磁盘
    /// - `key`: 配置项的键
    /// - `value`: 配置项的值，必须是可序列化为 JSON 的类型
    pub fn save(
        &mut self,
        section: impl Into<String>,
        key: impl Into<String>,
        value: serde_json::Value,
    ) -> Result<(), String> {
        let section = section.into();
        let key = key.into();
        self.data.entry(section).or_default().insert(key, value);
        let dir_path = self.config_dir()?;
        save_state_to_disk(&dir_path, "config.json", self)
    }
}
