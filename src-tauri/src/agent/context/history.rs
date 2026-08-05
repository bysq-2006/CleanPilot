use super::system_prompt::SystemPromptManager;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: AgentToolFunction,
}

/// 历史记录，注意，这里应当是队列中的其中一条的记录，而不是整个历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<AgentToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Clone)]
pub struct AgentHistory {
    pub system_prompt: Arc<Mutex<SystemPromptManager>>,
    pub inner: Arc<Mutex<Vec<AgentMessage>>>,
}

impl AgentHistory {
    /// history 内部只保存两类东西：system prompt manager 和非 system 的真实消息。
    pub fn new() -> Self {
        Self {
            system_prompt: Arc::new(Mutex::new(SystemPromptManager::new())),
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 这里只负责追加真实会话消息，不允许把 system prompt 当成普通消息塞进来。
    pub fn append_if_active(&self, token: &CancellationToken, message: AgentMessage) -> Result<bool, String> {
        let mut history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;
        if token.is_cancelled() {
            return Ok(false);
        }
        history.push(message);
        Ok(true)
    }

    /// 保留已生成的回复文本，并清理未完成的工具调用信息。
    pub fn preserve_cancelled_assistant_turn(&self) -> Result<(), String> {
        let mut history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;
        let last_user = history.iter().rposition(|message| message.role == "user");
        let last_assistant = history.iter().rposition(|message| message.role == "assistant");
        if let Some(index) = last_assistant.filter(|index| Some(*index) > last_user) {
            if history[index].tool_calls.is_some() {
                history.truncate(index + 1);
                history[index].tool_calls = None;
                if history[index].content.as_deref().unwrap_or_default().is_empty() {
                    history.pop();
                }
            }
        }
        Ok(())
    }

    /// 使用传入的闭包更新历史记录中的最后一条消息。
    pub fn update_last_message(&self, updater: impl FnOnce(&mut AgentMessage)) -> Result<(), String> {
        let mut history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;
        let message = history
            .last_mut()
            .ok_or_else(|| "Agent 历史记录为空，无法更新最后一条消息".to_string())?;
        updater(message);
        Ok(())
    }

}
