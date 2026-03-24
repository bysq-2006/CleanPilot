use std::sync::{Arc, Mutex};

use super::config::Config;
use crate::agent::runtime::AgentRuntime;

/// 全局状态管理，存储一些全局共享的数据
pub struct AppStore {
    pub agent: Arc<Mutex<Option<AgentRuntime>>>,
    pub config: Arc<Mutex<Config>>,
}

impl Default for AppStore {
    fn default() -> Self {
        let config = Arc::new(Mutex::new(Config::default()));

        Self {
            agent: Arc::new(Mutex::new(None)),
            config,
        }
    }
}

impl AppStore {
    pub fn init_agent(&self) -> Result<(), String> {
        let mut agent = self
            .agent
            .lock()
            .map_err(|e| format!("Agent 锁获取失败: {}", e))?;

        *agent = Some(AgentRuntime::new(Arc::clone(&self.config)));

        Ok(())
    }
}
