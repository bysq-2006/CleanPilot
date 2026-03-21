use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// 任务列表，这里应当是队列中的其中一条的记录，而不是整个历史记录
#[derive(Debug, Clone)]
pub enum AgentTask {
    /// UserQuestion 代表用户输入的一个问题，Agent 需要根据这个问题去思考下一步要调用哪个工具，或者直接回答
    UserQuestion { content: String },
    /// ToolCall 代表 Agent 决定要调用一个工具了
    ToolCall { tool_name: String, payload: String },
}