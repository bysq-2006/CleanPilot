/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务，并且他唯一的输出就是输出在 history 里面，不会直接和前端交互
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::time::Duration;

use super::context::history::AgentHistory;
use super::llm::AgentLlm;
use super::state::AgentState;
use super::tasks;
use super::tasks::queue::AgentTaskQueue;
use super::tools::ToolManager;
use crate::models::config::Config;
use crate::models::event_delegate::EventDelegate;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct AgentRuntime {
    pub history: AgentHistory,
    pub tasks: AgentTaskQueue,
    pub llm: AgentLlm,
    pub tools: Arc<Mutex<ToolManager>>,
    state: Arc<Mutex<AgentState>>,
    pub event_delegate: EventDelegate,
}

impl AgentRuntime {
    /// 创建 Agent 运行时及其共享组件。
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
            state: Arc::new(Mutex::new(AgentState::Idle)),
            event_delegate,
        }
    }

    /// 创建新请求并把用户问题加入任务队列。
    pub fn enqueue_user_question(&self, content: String) -> Result<(), String> {
        let mut state = self.state.lock()
            .map_err(|e| format!("Agent 状态锁获取失败: {}", e))?;
        let cancellation_token = state.begin()?;
        if let Err(error) = self.tasks.push(super::tasks::queue::AgentTask::UserQuestion {
            content,
            cancellation_token,
        }) {
            state.cancel();
            return Err(error);
        }
        Ok(())
    }

    /// 取消当前请求、清空待处理任务并回滚未完成回复。
    pub fn cancel_current(&self) -> Result<(), String> {
        let mut state = self.state.lock()
            .map_err(|e| format!("Agent 状态锁获取失败: {}", e))?;
        let cancelled = state.cancel();
        let clear_result = self.tasks.clear();
        let history_result = if cancelled {
            self.history.preserve_cancelled_assistant_turn()
        } else {
            Ok(())
        };

        clear_result?;
        history_result
    }

    /// 将属于当前请求的已完成令牌切换为空闲状态。
    pub fn finish_if_current(&self, token: &Arc<CancellationToken>) -> Result<(), String> {
        let mut state = self.state.lock()
            .map_err(|e| format!("Agent 状态锁获取失败: {}", e))?;
        state.finish_if_current(token);
        Ok(())
    }

    /// 读取 Agent 当前是否正在处理请求。
    pub fn is_chatting(&self) -> Result<bool, String> {
        let state = self.state.lock()
            .map_err(|e| format!("Agent 状态锁获取失败: {}", e))?;
        Ok(state.is_chatting())
    }

    /// 在 Tauri 异步运行时中启动任务消费循环。
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
