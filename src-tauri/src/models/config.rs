use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::models::llm_config::LlmConfig;
use crate::utils::state_file::{load_state_from_disk, save_state_to_disk};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub config_file_path: PathBuf,
    #[serde(default)]
    pub llm: LlmConfig,
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
        self.llm = disk_config.llm;
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

    pub fn get_section(&self, section: &str) -> Result<serde_json::Value, String> {
        match section {
            "llm" => serde_json::to_value(&self.llm).map_err(|e| format!("llm 配置序列化失败: {}", e)),
            _ => Err(format!("未知配置分区: {}", section)),
        }
    }

    pub fn save_section(&mut self, section: &str, value: serde_json::Value) -> Result<(), String> {
        match section {
            "llm" => {
                self.llm = serde_json::from_value(value)
                    .map_err(|e| format!("llm 配置反序列化失败: {}", e))?;
            }
            _ => return Err(format!("未知配置分区: {}", section)),
        }

        let dir_path = self.config_dir()?;
        save_state_to_disk(&dir_path, "config.json", self)
    }
}
