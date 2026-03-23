use super::system_prompt::SystemPromptManager;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// 给 LLM 的最终输入形状固定为 system + context，history 不应输出别的格式。
#[derive(Debug, Clone, Serialize)]
struct AgentHistoryLlmInput {
    system: String,
    context: Vec<AgentHistoryContextItem>,
}

/// context 内部目前只保留最小字段，后面扩展 tool_calls 时再加字段。
#[derive(Debug, Clone, Serialize)]
struct AgentHistoryContextItem {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<AgentToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: AgentToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolFunction {
    pub name: String,
    pub arguments: String,
}

/// 历史记录，注意，这里应当是队列中的其中一条的记录，而不是整个历史记录
#[derive(Debug, Clone, Serialize)]
pub struct AgentMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
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
    pub fn new(system_prompt: SystemPromptManager) -> Self {
        Self {
            system_prompt: Arc::new(Mutex::new(system_prompt)),
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 这里只负责追加真实会话消息，不允许把 system prompt 当成普通消息塞进来。
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

    /// 唯一正式导出接口：必须返回完整 JSON 字符串，供 LLM 直接消费。
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
                    tool_calls: message.tool_calls.clone(),
                    tool_call_id: message.tool_call_id.clone(),
                })
                .collect(),
        };

        serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Agent LLM 输入序列化失败: {}", e))
    }

}
