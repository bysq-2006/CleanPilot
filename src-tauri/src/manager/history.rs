use std::{fs, path::PathBuf, sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use serde::{Deserialize, Serialize};

use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::agent::context::history::AgentMessage;
use crate::manager::agent_scene::AgentSceneManager;

const HISTORY_DIR_NAME: &str = "history";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecordSummary {
    pub context_id: String,
    pub scene: String,
    #[serde(default)]
    pub updated_at: u64,
    pub message_count: usize,
    pub preview: String,
    #[serde(default)]
    pub items: Vec<AgentMessage>,
}

/// 历史记录管理器。
///
/// 当前只定义上下文切换与路径定位所需的最小骨架：
/// - 当前活跃上下文 ID
/// - 历史记录目录路径
#[derive(Debug, Clone, Default)]
pub struct HistoryManager {
    pub agent_scene: Arc<AgentSceneManager>,
    pub current_context_id: Arc<Mutex<Option<String>>>,
    pub history_dir_path: Arc<Mutex<PathBuf>>,
}

impl HistoryManager {
    pub fn new(app: &AppHandle, agent_scene: Arc<AgentSceneManager>) -> Result<Self, String> {
        Ok(Self {
            agent_scene,
            current_context_id: Arc::new(Mutex::new(Some(Uuid::new_v4().to_string()))),
            history_dir_path: Arc::new(Mutex::new(
                app.path()
                    .app_data_dir()
                    .map_err(|e| format!("无法获取应用数据目录: {}", e))?
                    .join(HISTORY_DIR_NAME),
            )),
        })
    }

    /// 设置当前活跃的上下文 ID，这通常在用户切换会话时调用。
    pub fn set_current_context_id(&self, context_id: String) -> Result<(), String> {
        let mut current_context_id = self
            .current_context_id
            .lock()
            .map_err(|e| format!("当前上下文锁获取失败: {}", e))?;

        *current_context_id = Some(context_id);

        Ok(())
    }

    /// 获取当前活跃的上下文 ID，如果没有设置则返回 None。
    pub fn get_current_context_id(&self) -> Result<Option<String>, String> {
        self
            .current_context_id
            .lock()
            .map(|id| id.clone())
            .map_err(|e| format!("当前上下文锁获取失败: {}", e))
    }

    /// 获取当前上下文对应的历史记录文件路径，如果当前上下文未设置则返回 None。
    pub fn get_current_context_path(&self) -> Result<Option<PathBuf>, String> {
        let current_context_id = self.get_current_context_id()?;
        let Some(current_context_id) = current_context_id else {
            return Ok(None);
        };

        let history_dir_path = self
            .history_dir_path
            .lock()
            .map_err(|e| format!("历史目录锁获取失败: {}", e))?;

        if history_dir_path.as_os_str().is_empty() {
            return Err("历史目录未初始化，请先创建 HistoryManager".to_string());
        }

        Ok(Some(history_dir_path.join(format!("{}.json", current_context_id))))
    }

    /// 保存历史记录上下文到文件系统，供后续读取。
    pub fn save_context_items(&self, items: &Vec<AgentMessage>) -> Result<(), String> {
        if items.is_empty() {
            return Ok(());
        }

        let context_file_path = self
            .get_current_context_path()?
            .ok_or_else(|| "当前上下文未设置，无法保存历史记录".to_string())?;

        let history_dir_path = self
            .history_dir_path
            .lock()
            .map_err(|e| format!("历史目录锁获取失败: {}", e))?;

        fs::create_dir_all(history_dir_path.as_path())
            .map_err(|e| format!("创建历史目录失败: {}", e))?;

        let context_id = self
            .get_current_context_id()?
            .ok_or_else(|| "当前上下文未设置，无法保存历史记录".to_string())?;
        let scene = self.agent_scene.get_current_scene()?.as_str().to_string();
        let preview = items
            .iter()
            .find_map(|item| item.content.as_ref())
            .map(|content| content.trim().to_string())
            .filter(|content| !content.is_empty())
            .unwrap_or_else(|| "空白会话".to_string());
        let record = HistoryRecordSummary {
            context_id,
            scene,
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| format!("系统时间异常: {}", e))?
                .as_secs(),
            message_count: items.len(),
            preview,
            items: items.clone(),
        };

        let content = serde_json::to_string_pretty(&record)
            .map_err(|e| format!("历史记录序列化失败: {}", e))?;

        fs::write(context_file_path, content)
            .map_err(|e| format!("写入历史记录失败: {}", e))?;

        Ok(())
    }

    /// 从文件系统加载历史记录上下文，供 AgentRuntime 恢复会话状态。
    pub fn load_context_items(&self) -> Result<HistoryRecordSummary, String> {
        let context_file_path = self
            .get_current_context_path()?
            .ok_or_else(|| "当前上下文未设置，无法读取历史记录".to_string())?;

        let content = fs::read_to_string(context_file_path)
            .map_err(|e| format!("读取历史记录失败: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("历史记录反序列化失败: {}", e))
    }

    pub fn delete_context(&self, context_id: String) -> Result<(), String> {
        let history_dir_path = self
            .history_dir_path
            .lock()
            .map_err(|e| format!("历史目录锁获取失败: {}", e))?
            .clone();

        let context_file_path = history_dir_path.join(format!("{}.json", context_id));

        if !context_file_path.exists() {
            return Ok(());
        }

        fs::remove_file(context_file_path)
            .map_err(|e| format!("删除历史记录失败: {}", e))?;

        Ok(())
    }

    /// 列出所有历史记录文件，返回它们的上下文 ID、消息数量和预览信息，供 UI 展示会话列表。
    pub fn list_history_records(&self) -> Result<Vec<HistoryRecordSummary>, String> {
        let history_dir_path = self
            .history_dir_path
            .lock()
            .map_err(|e| format!("历史目录锁获取失败: {}", e))?
            .clone();

        let mut records = Vec::new();

        for entry in fs::read_dir(&history_dir_path)
            .map_err(|e| format!("读取历史目录失败: {}", e))?
        {
            let entry = entry.map_err(|e| format!("遍历历史目录失败: {}", e))?;
            let path = entry.path();
            let context_id = path
                .file_stem()
                .and_then(|value| value.to_str())
                .map(|value| value.to_string())
                .ok_or_else(|| "历史记录文件名无效".to_string())?;

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("读取历史记录文件失败: {}", e))?;
            let record: HistoryRecordSummary = serde_json::from_str(&content)
                .map_err(|e| format!("历史记录反序列化失败: {}", e))?;

            records.push(HistoryRecordSummary {
                context_id,
                scene: record.scene,
                updated_at: record.updated_at,
                message_count: record.message_count,
                preview: record.preview,
                items: record.items,
            });
        }

        records.sort_by(|a, b| {
            b.updated_at
                .cmp(&a.updated_at)
                .then_with(|| b.context_id.cmp(&a.context_id))
        });

        Ok(records)
    }
}
