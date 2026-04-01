use std::sync::Arc;

use tauri::AppHandle;

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
}
