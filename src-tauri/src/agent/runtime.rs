/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务，并且他唯一的输出就是输出在 history 里面，不会直接和前端交互
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::sync::Arc;
use std::time::Duration;

use super::history::AgentHistory;
use super::tasks;
use super::task_queue::AgentTaskQueue;
use crate::llm::LlmService;
use tokio::time::sleep;

#[derive(Clone)]
pub struct AgentRuntime {
    pub history: AgentHistory,
    pub tasks: AgentTaskQueue,
    pub llm: LlmService,
}

impl AgentRuntime {
    pub fn new(llm: LlmService) -> Self {
        Self {
            history: AgentHistory::new(
                "你是 CleanPilot 的系统级 Agent。请遵守系统规则，基于历史记录思考，并输出安全、简洁、可执行的下一步。".to_string(),
            ),
            tasks: AgentTaskQueue::default(),
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
