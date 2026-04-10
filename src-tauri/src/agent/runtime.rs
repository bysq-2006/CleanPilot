/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务，并且他唯一的输出就是输出在 history 里面，不会直接和前端交互
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::time::Duration;

use super::context::history::AgentHistory;
use super::llm::AgentLlm;
use super::tasks;
use super::tasks::queue::AgentTaskQueue;
use super::tools::ToolManager;
use crate::models::config::Config;
use crate::models::event_delegate::EventDelegate;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Chatting,
}

#[derive(Clone)]
pub struct AgentRuntime {
    pub history: AgentHistory,
    pub tasks: AgentTaskQueue,
    pub llm: AgentLlm,
    pub tools: Arc<Mutex<ToolManager>>,
    pub status: Arc<Mutex<AgentStatus>>,
    pub event_delegate: EventDelegate,
}

impl AgentRuntime {
    pub fn new(
        config: Arc<Mutex<Config>>,
        event_delegate: EventDelegate,
    ) -> Self {
        let tools = ToolManager::new("*");
        let history = AgentHistory::new();
        history
            .system_prompt
            .lock()
            .expect("初始化 Agent system prompt 锁失败")
            .set_tool_prompt(tools.build_prompt());

        Self {
            history,
            tasks: AgentTaskQueue::default(),
            llm: AgentLlm::new(config),
            tools: Arc::new(Mutex::new(tools)),
            status: Arc::new(Mutex::new(AgentStatus::Idle)),
            event_delegate,
        }
    }

    pub fn set_status(&self, status: AgentStatus) -> Result<(), String> {
        let mut current_status = self
            .status
            .lock()
            .map_err(|e| format!("Agent 状态锁获取失败: {}", e))?;
        *current_status = status;
        Ok(())
    }

    pub fn start(&self) {
        let runtime = self.clone();
        tauri::async_runtime::spawn(async move {
            runtime.run_loop().await;
        });
    }

    /// 主循环只消费 task queue，所有中间状态都落到 history，不直接面向前端输出。
    async fn run_loop(self) {
        loop {
            let next_task = self.tasks.pop().unwrap_or_else(|e| {
                eprintln!("{}", e);
                None
            });

            match next_task {
                Some(task) => tasks::handle_task(&self, task).await,
                None => sleep(Duration::from_millis(100)).await,
            }
        }
    }
}
