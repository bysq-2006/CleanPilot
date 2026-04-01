use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tokio::time::sleep;

use crate::models::appstore::AppStore;

pub mod agent_scene;
pub mod history;

use self::agent_scene::AgentSceneManager;
use self::history::HistoryManager;

/// 管理模块入口。
#[derive(Debug, Clone, Default)]
pub struct ManagerModule {
    pub agent_scene: Arc<AgentSceneManager>,
    pub history: Arc<HistoryManager>,
}

impl ManagerModule {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let agent_scene = Arc::new(AgentSceneManager::new());

        Ok(Self {
            history: Arc::new(HistoryManager::new(app, Arc::clone(&agent_scene))?),
            agent_scene,
        })
    }

    pub fn start_auto_save(&self, app: AppHandle) {
        let manager = Arc::clone(&app.state::<AppStore>().manager);

        tauri::async_runtime::spawn(async move {
            loop {
                sleep(Duration::from_secs(3)).await;

                let store = app.state::<AppStore>();
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
        });
    }
}
