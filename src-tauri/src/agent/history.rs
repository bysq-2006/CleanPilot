use super::system_prompt::SystemPromptManager;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
struct AgentHistoryLlmInput {
    system: String,
    context: Vec<AgentHistoryContextItem>,
}

#[derive(Debug, Clone, Serialize)]
struct AgentHistoryContextItem {
    #[serde(rename = "type")]
    message_type: String,
    content: String,
}

/// 历史记录，注意，这里应当是队列中的其中一条的记录，而不是整个历史记录
#[derive(Debug, Clone, Serialize)]
pub struct AgentMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct AgentHistory {
    pub system_prompt: Arc<Mutex<SystemPromptManager>>,
    pub inner: Arc<Mutex<Vec<AgentMessage>>>,
}

impl AgentHistory {
    pub fn new(system_prompt: SystemPromptManager) -> Self {
        Self {
            system_prompt: Arc::new(Mutex::new(system_prompt)),
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn append(&self, message: AgentMessage) -> Result<(), String> {
        let mut history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;
        history.push(message);
        Ok(())
    }

    /// 获取系统提示词的完整内容
    pub fn get_system_prompt(&self) -> Result<String, String> {
        let system_prompt = self
            .system_prompt
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

        Ok(system_prompt.build())
    }

    pub fn update_system_prompt<F>(&self, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut SystemPromptManager),
    {
        let mut system_prompt = self
            .system_prompt
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

        updater(&mut system_prompt);
        Ok(())
    }

    pub fn build_llm_input(&self) -> Result<String, String> {
        let system_prompt = self.get_system_prompt()?;
        let history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

        let payload = AgentHistoryLlmInput {
            system: system_prompt,
            context: history
                .iter()
                .map(|message| AgentHistoryContextItem {
                    message_type: message.role.clone(),
                    content: message.content.clone(),
                })
                .collect(),
        };

        serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Agent LLM 输入序列化失败: {}", e))
    }

}
