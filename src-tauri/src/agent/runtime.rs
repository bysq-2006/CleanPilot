/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务，并且他唯一的输出就是输出在 history 里面，不会直接和前端交互
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::time::Duration;

use super::history::AgentHistory;
use super::tasks;
use super::tasks::queue::AgentTaskQueue;
use super::tools::ToolManager;
use crate::llm::LlmService;
use tokio::time::sleep;

#[derive(Clone)]
pub struct AgentRuntime {
    pub history: AgentHistory,
    pub tasks: AgentTaskQueue,
    pub llm: LlmService,
    pub tools: ToolManager,
}

impl AgentRuntime {
    pub fn new(llm: LlmService) -> Self {
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
            llm,
            tools,
        }
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
