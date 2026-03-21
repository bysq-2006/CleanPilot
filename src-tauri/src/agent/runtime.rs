/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::sync::Arc;
use std::time::Duration;
use std::{collections::VecDeque, sync::Mutex};

use super::history::{AgentMessage, AgentMessageRole};
use super::task_queue::AgentTask;
use crate::llm::LlmService;
use tokio::time::sleep;

#[derive(Clone)]
pub struct AgentRuntime {
    pub history: Arc<Mutex<Vec<AgentMessage>>>,
    pub tasks: Arc<Mutex<VecDeque<AgentTask>>>,
    pub llm: LlmService,
}

impl AgentRuntime {
    pub fn new(llm: LlmService) -> Self {
        Self {
            history: Arc::new(Mutex::new(vec![AgentMessage {
                role: AgentMessageRole::System,
                content: "你是 CleanPilot 的系统级 Agent。请遵守系统规则，基于历史记录思考，并输出安全、简洁、可执行的下一步。".to_string(),
            }])),
            tasks: Arc::default(),
            llm,
        }
    }

    pub fn start(&self) {
        let runtime = self.clone();
        tauri::async_runtime::spawn(async move {
            runtime.run_loop().await;
        });
    }

    /// 主要循环，负责轮询任务队列并执行任务
    async fn run_loop(self) {
        loop {
            let next_task = match self.tasks.lock() {
                Ok(mut tasks) => tasks.pop_front(),
                Err(e) => {
                    eprintln!("Agent 任务队列加锁失败: {}", e);
                    None
                }
            };

            match next_task {
                Some(task) => self.handle_task(task).await,
                None => sleep(Duration::from_millis(100)).await,
            }
        }
    }

    /// 处理单个任务，根据任务类型调用不同的处理逻辑
    async fn handle_task(&self, task: AgentTask) {
        match task {
            AgentTask::UserQuestion { content } => {
                if let Err(e) = self.append_history(AgentMessage {
                    role: AgentMessageRole::User,
                    content: content.clone(),
                }) {
                    eprintln!("Agent 写入历史记录失败: {}", e);
                }
                println!("Agent 收到用户问题任务: {}", content);
            }
            AgentTask::ToolCall { tool_name, payload } => {
                println!("Agent 收到工具调用任务: {}, payload={}", tool_name, payload);
            }
        }
    }

    /// 压入一个任务，类型要根据 AgentTask
    pub fn push_task(&self, task: AgentTask) -> Result<(), String> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Agent 任务队列加锁失败: {}", e))?;
        tasks.push_back(task);
        Ok(())
    }

    /// 压入一条历史记录，类型要根据 AgentMessage
    fn append_history(&self, message: AgentMessage) -> Result<(), String> {
        let mut history = self
            .history
            .lock()
            .map_err(|e| format!("Agent 历史记录加锁失败: {}", e))?;
        history.push(message);
        Ok(())
    }

    pub fn get_system_prompt(&self) -> Result<String, String> {
        let history = self
            .history
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
            .history
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
