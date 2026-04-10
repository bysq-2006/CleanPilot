use std::sync::{Arc, Mutex};

use tauri::AppHandle;

use super::config::Config;
use super::event_delegate::EventDelegate;
use crate::agent::runtime::AgentRuntime;
use crate::manager::ManagerModule;

/// 全局状态管理，存储一些全局共享的数据
pub struct AppStore {
    pub agent: Arc<Mutex<Option<AgentRuntime>>>,
    pub config: Arc<Mutex<Config>>,
    pub manager: Arc<ManagerModule>,
    pub event_delegate: EventDelegate,
}

impl AppStore {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let config = Arc::new(Mutex::new(Config::default()));

        Ok(Self {
            agent: Arc::new(Mutex::new(None)),
            config,
            manager: Arc::new(ManagerModule::new(app)?),
            event_delegate: EventDelegate::new(32),
        })
    }

    pub fn init_agent(&self) -> Result<(), String> {
        let mut agent = self
            .agent
            .lock()
            .map_err(|e| format!("Agent 锁获取失败: {}", e))?;

        let runtime = AgentRuntime::new(Arc::clone(&self.config));
        self.manager
            .agent_scene
            .switch_scene(
                self.manager.agent_scene.get_current_scene()?,
                &runtime,
            )?;

        *agent = Some(runtime);

        Ok(())
    }
}
