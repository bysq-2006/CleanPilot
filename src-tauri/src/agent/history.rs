use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub enum AgentMessageRole {
    System,
    User,
    Assistant,
}

impl AgentMessageRole {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

/// 历史记录，注意，这里应当是队列中的其中一条的记录，而不是整个历史记录
#[derive(Debug, Clone, Serialize)]
pub struct AgentMessage {
    pub role: AgentMessageRole,
    pub content: String,
}

#[derive(Clone)]
pub struct AgentHistory {
    inner: Arc<Mutex<Vec<AgentMessage>>>,
}

impl AgentHistory {
    pub fn new(system_prompt: String) -> Self {
        Self {
            inner: Arc::new(Mutex::new(vec![AgentMessage {
                role: AgentMessageRole::System,
                content: system_prompt,
            }])),
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

    pub fn get_from(&self, start_index: usize) -> Result<Vec<AgentMessage>, String> {
        let history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

        if start_index >= history.len() {
            return Ok(Vec::new());
        }

        Ok(history[start_index..].to_vec())
    }

    pub fn get_system_prompt(&self) -> Result<String, String> {
        let history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

        history
            .iter()
            .find(|message| matches!(message.role, AgentMessageRole::System))
            .map(|message| message.content.clone())
            .ok_or_else(|| "Agent 系统提示词不存在".to_string())
    }

    pub fn set_system_prompt(&self, prompt: String) -> Result<(), String> {
        let mut history = self
            .inner
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;

        if let Some(message) = history
            .iter_mut()
            .find(|message| matches!(message.role, AgentMessageRole::System))
        {
            message.content = prompt;
            return Ok(());
        }

        history.insert(
            0,
            AgentMessage {
                role: AgentMessageRole::System,
                content: prompt,
            },
        );

        Ok(())
    }
}

impl AgentMessage {
    /// 序列化成字符串，格式为 {"role": "user", "content": "用户输入的内容"}
    #[allow(dead_code)]
    pub fn serialize(&self) -> String {
        format!(
            "{{\"role\": \"{}\", \"content\": \"{}\"}}",
            self.role.as_str(),
            self.content.replace('"', "\\\"")
        )
    }
}

/// 将历史记录列表序列化成可直接发送给 OpenAI / DeepSeek 的 messages JSON 字符串
#[allow(dead_code)]
pub fn serialize_history(messages: &[AgentMessage]) -> String {
    let serialized_items = messages
        .iter()
        .map(AgentMessage::serialize)
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{}]", serialized_items)
}

