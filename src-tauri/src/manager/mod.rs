use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tokio::time::sleep;

use crate::agent::runtime::AgentStatus;
use crate::commands::storage_box::DiskCleanupItem;
use crate::models::appstore::AppStore;
use crate::models::event_delegate::EventDelegate;

pub mod agent_scene;
pub mod history;
pub mod storage_box;

use self::agent_scene::AgentSceneManager;
use self::history::HistoryManager;
use self::storage_box::StorageBoxManager;

/// 管理模块入口。
#[derive(Debug, Clone)]
pub struct ManagerModule {
    pub agent_scene: Arc<AgentSceneManager>,
    pub history: Arc<HistoryManager>,
    pub storage_box: Arc<StorageBoxManager>,
}

impl ManagerModule {
    pub fn new(app: &AppHandle, event_delegate: EventDelegate) -> Result<Self, String> {
        let agent_scene = Arc::new(AgentSceneManager::new());
        let storage_box = Arc::new(StorageBoxManager::new(app)?);

        let manager = Self {
            history: Arc::new(HistoryManager::new(app, Arc::clone(&agent_scene))?),
            agent_scene,
            storage_box: Arc::clone(&storage_box),
        };

        manager.start_event_delegate_listener(event_delegate);

        Ok(manager)
    }

    /// 启动自动保存功能，定期检查 Agent 状态并保存历史记录。
    pub fn start_auto_save(&self, app: AppHandle) {
        let manager = Arc::clone(&app.state::<AppStore>().manager);

        tauri::async_runtime::spawn(async move {
            let store = app.state::<AppStore>();
            let mut last_status = None;

            loop {
                sleep(Duration::from_millis(500)).await;

                let agent_guard = match store.agent.lock() {
                    Ok(agent) => agent,
                    Err(error) => {
                        eprintln!("Agent 锁获取失败: {}", error);
                        continue;
                    }
                };

                let Some(agent) = agent_guard.as_ref() else {
                    continue;
                };

                let current_status = match agent.status.lock() {
                    Ok(status) => *status,
                    Err(error) => {
                        eprintln!("Agent 状态锁获取失败: {}", error);
                        continue;
                    }
                };

                if last_status == Some(current_status) {
                    continue;
                }

                last_status = Some(current_status);

                if current_status == AgentStatus::Idle || current_status == AgentStatus::Chatting {
                    let history = match agent.history.inner.lock() {
                        Ok(history) => history,
                        Err(error) => {
                            eprintln!("Agent 历史记录加锁失败: {}", error);
                            continue;
                        }
                    };

                    if let Err(error) = manager.history.save_context_items(&history) {
                        eprintln!("自动保存历史记录失败: {}", error);
                    }
                }
            }
        });
    }

    pub fn start_event_delegate_listener(&self, event_delegate: EventDelegate) {
        let manager = self.clone();

        tauri::async_runtime::spawn(async move {
            let receiver = {
                let mut receiver_guard = match event_delegate.receiver.lock() {
                    Ok(receiver) => receiver,
                    Err(error) => {
                        eprintln!("事件委托接收器加锁失败: {}", error);
                        return;
                    }
                };

                match receiver_guard.take() {
                    Some(receiver) => receiver,
                    None => {
                        eprintln!("事件委托接收器已被占用，跳过重复监听");
                        return;
                    }
                }
            };

            manager.run_event_delegate_listener(receiver).await;
        });
    }

    async fn run_event_delegate_listener(
        self,
        mut receiver: tokio::sync::mpsc::Receiver<String>,
    ) {
        while let Some(message) = receiver.recv().await {
            if let Err(error) = self.handle_event_delegate_message(&message) {
                eprintln!("处理工具事件委托消息失败: {}", error);
            }
        }
    }

    fn handle_event_delegate_message(&self, message: &str) -> Result<(), String> {
        let payload: serde_json::Value = serde_json::from_str(message)
            .map_err(|e| format!("事件委托消息解析失败: {}", e))?;

        let event_name = payload
            .get("event")
            .and_then(|value| value.as_str())
            .ok_or_else(|| "事件委托消息缺少 event 字段".to_string())?;

        match event_name {
            "write_storage_box_checklist" => self.handle_write_storage_box_checklist_event(&payload),
            _ => Err(format!("未知事件委托类型: {}", event_name)),
        }
    }

    fn handle_write_storage_box_checklist_event(&self, payload: &serde_json::Value) -> Result<(), String> {
        let content_value = payload
            .get("content")
            .cloned()
            .ok_or_else(|| "write_storage_box_checklist 缺少 content 字段".to_string())?;

        let items: Vec<DiskCleanupItem> = serde_json::from_value(content_value)
            .map_err(|e| format!("write_storage_box_checklist content 解析失败: {}", e))?;

        if items.is_empty() {
            return Err("write_storage_box_checklist content 不能为空".to_string());
        }

        self.storage_box.save_new_record(
            build_disk_cleanup_file_name(),
            serde_json::to_value(items)
                .map_err(|e| format!("disk cleanup content 序列化失败: {}", e))?,
            "disk_cleanup".to_string(),
        )
    }
}

fn build_disk_cleanup_file_name() -> String {
    let saved_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);

    format!("disk-cleanup-{}.json", saved_at)
}
