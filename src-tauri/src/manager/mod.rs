use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tokio::time::sleep;

use crate::agent::runtime::AgentStatus;
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

        let receiver = event_delegate
            .receiver
            .lock()
            .map_err(|e| format!("事件委托接收器加锁失败: {}", e))?
            .take()
            .ok_or_else(|| "事件委托接收器已被占用".to_string())?;

        let manager_clone = manager.clone();
        tauri::async_runtime::spawn(async move {
            manager_clone.run_event_delegate_listener(receiver).await;
        });

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

    async fn run_event_delegate_listener(
        self,
        mut receiver: tokio::sync::mpsc::Receiver<String>,
    ) {
        while let Some(message) = receiver.recv().await {
            let result = serde_json::from_str::<serde_json::Value>(&message)
                .map_err(|e| format!("事件委托消息解析失败: {}", e))
                .and_then(|payload| {
                    match payload.get("event").and_then(|v| v.as_str()) {
                        Some("write_storage_box_checklist") => self.on_write_storage_box_checklist(&payload),
                        Some(name) => Err(format!("未知事件委托类型: {}", name)),
                        None => Err("事件委托消息缺少 event 字段".to_string()),
                    }
                });

            if let Err(error) = result {
                eprintln!("处理工具事件委托消息失败: {}", error);
            }
        }
    }

    fn on_write_storage_box_checklist(&self, payload: &serde_json::Value) -> Result<(), String> {
        let title = payload
            .get("title")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| "write_storage_box_checklist title 缺失或为空".to_string())?;

        let content = payload
            .get("content")
            .and_then(|v| v.as_array())
            .filter(|arr| !arr.is_empty())
            .cloned()
            .ok_or_else(|| "write_storage_box_checklist content 缺失或为空".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.storage_box.save_new_record(
            format!("{}-{}.json", title, timestamp),
            serde_json::Value::Array(content),
            "disk_cleanup".to_string(),
        )
    }
}
