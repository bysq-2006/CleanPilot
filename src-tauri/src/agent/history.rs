use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum AgentMessageRole {
    System,
    User,
    Assistant,
}

impl AgentMessageRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

/// 历史记录，注意，这里应当是队列中的其中一条的记录，而不是整个历史记录
#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub role: AgentMessageRole,
    pub content: String,
}

impl AgentMessage {
    /// 序列化成字符串，格式为 {"role": "user", "content": "用户输入的内容"}
    pub fn serialize(&self) -> String {
        format!(
            "{{\"role\": \"{}\", \"content\": \"{}\"}}",
            self.role.as_str(),
            self.content.replace('"', "\\\"")
        )
    }
}

/// 将历史记录列表序列化成可直接发送给 OpenAI / DeepSeek 的 messages JSON 字符串
pub fn serialize_history(messages: &[AgentMessage]) -> String {
    let serialized_items = messages
        .iter()
        .map(AgentMessage::serialize)
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{}]", serialized_items)
}

