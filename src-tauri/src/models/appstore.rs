use std::sync::{Arc, Mutex};

use super::config::Config;
use crate::agent::runtime::AgentRuntime;
use crate::llm::LlmService;

/// 全局状态管理，存储一些全局共享的数据
pub struct AppStore {
    pub agent: AgentRuntime,
    pub config: Arc<Mutex<Config>>,
}

impl Default for AppStore {
    fn default() -> Self {
        let config = Arc::new(Mutex::new(Config::default()));

        Self {
            agent: AgentRuntime::new(LlmService::new(Arc::clone(&config))),
            config,
        }
    }
}

impl AppStore {
}
