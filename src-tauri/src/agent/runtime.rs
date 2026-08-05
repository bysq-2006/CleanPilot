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
use tokio_util::sync::CancellationToken;

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
    pub cancellation_token: Arc<Mutex<Option<Arc<CancellationToken>>>>,
    pub event_delegate: EventDelegate,
}

impl AgentRuntime {
    pub fn new(
        config: Arc<Mutex<Config>>,
        event_delegate: EventDelegate,
    ) -> Self {
        let tools = ToolManager::new("*");
        let history = AgentHistory::new();

        Self {
            history,
            tasks: AgentTaskQueue::default(),
            llm: AgentLlm::new(config),
            tools: Arc::new(Mutex::new(tools)),
            status: Arc::new(Mutex::new(AgentStatus::Idle)),
            cancellation_token: Arc::new(Mutex::new(None)),
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

    pub fn enqueue_user_question(&self, content: String) -> Result<(), String> {
        let mut status = self.status.lock()
            .map_err(|e| format!("Agent 状态锁获取失败: {}", e))?;
        if *status == AgentStatus::Chatting {
            return Err("Agent 正在处理上一条消息".to_string());
        }

        let cancellation_token = Arc::new(CancellationToken::new());
        self.tasks.push(super::tasks::queue::AgentTask::UserQuestion {
            content,
            cancellation_token: cancellation_token.clone(),
        })?;
        *self.cancellation_token.lock()
            .map_err(|e| format!("Agent 取消令牌锁获取失败: {}", e))? = Some(cancellation_token);
        *status = AgentStatus::Chatting;
        Ok(())
    }

    pub fn cancel_current(&self) -> Result<(), String> {
        let cancelled = if let Some(token) = self.cancellation_token.lock()
            .map_err(|e| format!("Agent 取消令牌锁获取失败: {}", e))?
            .take()
        {
            token.cancel();
            true
        } else {
            false
        };

        let clear_result = self.tasks.clear();
        if cancelled {
            self.history.discard_incomplete_assistant_turn()?;
        }
        self.set_status(AgentStatus::Idle)?;
        clear_result
    }

    pub fn finish_if_current(&self, token: &Arc<CancellationToken>) -> Result<(), String> {
        let mut current = self.cancellation_token.lock()
            .map_err(|e| format!("Agent 取消令牌锁获取失败: {}", e))?;
        if current.as_ref().is_some_and(|active| Arc::ptr_eq(active, token)) {
            *current = None;
            self.set_status(AgentStatus::Idle)?;
        }
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
                log::error!("{}", e);
                None
            });

            match next_task {
                Some(task) => tasks::handle_task(&self, task).await,
                None => sleep(Duration::from_millis(100)).await,
            }
        }
    }
}
