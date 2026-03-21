/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::sync::Arc;
use std::time::Duration;

use super::types::{AgentTask, AgentTaskQueue};
use tokio::time::sleep;

#[derive(Clone)]
pub struct AgentRuntime {
    pub tasks: AgentTaskQueue,
}

impl Default for AgentRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRuntime {
    pub fn new() -> Self {
        Self {
            tasks: Arc::default(),
        }
    }

    pub fn start(&self) {
        let runtime = self.clone();
        tauri::async_runtime::spawn(async move {
            runtime.run_loop().await;
        });
    }

    /// 主要循环
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

    pub fn push_task(&self, task: AgentTask) -> Result<(), String> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Agent 任务队列加锁失败: {}", e))?;
        tasks.push_back(task);
        Ok(())
    }

    async fn handle_task(&self, task: AgentTask) {
        match task {
            AgentTask::UserQuestion { content } => {
                println!("Agent 收到用户问题任务: {}", content);
            }
            AgentTask::ToolCall { tool_name, payload } => {
                println!("Agent 收到工具调用任务: {}, payload={}", tool_name, payload);
            }
        }
    }
}
