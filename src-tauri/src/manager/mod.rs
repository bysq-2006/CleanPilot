use std::sync::Arc;

use tauri::{AppHandle, Manager};

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
        let events = {
            let store = app.state::<AppStore>();
            let agent_guard = match store.agent.lock() {
                Ok(agent) => agent,
                Err(error) => {
                    eprintln!("Agent 锁获取失败: {}", error);
                    return;
                }
            };

            let Some(agent) = agent_guard.as_ref() else {
                return;
            };

            agent.events.subscribe()
        };

        tauri::async_runtime::spawn(async move {
            let store = app.state::<AppStore>();
            let mut events = events;

            loop {
                match events.recv().await {
                    Ok(event) if event == "chat_round_finished" => {
                        if let Err(error) = save_agent_history(&store, &manager) {
                            eprintln!("自动保存历史记录失败: {}", error);
                        }
                    }
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Agent 事件监听失败: {}", error);
                    }
                }
            }
        });
    }
}

/// 从 AgentRuntime 中获取历史记录并保存到文件系统。
fn save_agent_history(store: &tauri::State<'_, AppStore>, manager: &Arc<ManagerModule>) -> Result<(), String> {
    let agent_guard = store
        .agent
        .lock()
        .map_err(|error| format!("Agent 锁获取失败: {}", error))?;

    let agent = agent_guard
        .as_ref()
        .ok_or_else(|| "Agent 尚未初始化".to_string())?;

    let history = agent
        .history
        .inner
        .lock()
        .map_err(|error| format!("Agent 历史记录加锁失败: {}", error))?;

    manager.history.save_context_items(&history)
}
