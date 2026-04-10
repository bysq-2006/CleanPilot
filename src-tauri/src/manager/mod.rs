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
    pub event_delegate: EventDelegate,
}

impl ManagerModule {
    pub fn new(app: &AppHandle, event_delegate: EventDelegate) -> Result<Self, String> {
        let agent_scene = Arc::new(AgentSceneManager::new());
        let storage_box = Arc::new(StorageBoxManager::new(app)?);

        Ok(Self {
            history: Arc::new(HistoryManager::new(app, Arc::clone(&agent_scene))?),
            agent_scene,
            storage_box: Arc::clone(&storage_box),
            event_delegate,
        })
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
}
