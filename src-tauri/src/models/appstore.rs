use std::sync::Mutex;

use super::config::Config;
use crate::agent::runtime::AgentRuntime;
use crate::llm;
use crate::llm::types::LlmResponse;

/// 全局状态管理，存储一些全局共享的数据
#[derive(Default)]
pub struct AppStore {
    pub agent: AgentRuntime,
    pub config: Mutex<Config>,
    pub ui_message_history: Mutex<Vec<String>>,
}

impl AppStore {
}
