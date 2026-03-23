/// 设计哲学是：本身有一个循环，然后循环要根据任务列表去执行任务，并且他唯一的输出就是输出在 history 里面，不会直接和前端交互
/// 然后每一任务都是一个单独的文件，任务的执行逻辑也在那个文件里
/// 而这一层就是要根据match去调用不同的任务文件里的函数
use std::time::Duration;

use super::history::AgentHistory;
use super::system_prompt::SystemPromptManager;
use super::tasks;
use super::task_queue::AgentTaskQueue;
use crate::llm::LlmService;
use tokio::time::sleep;

#[derive(Clone)]
pub struct AgentRuntime {
    pub history: AgentHistory,
    pub tasks: AgentTaskQueue,
    pub llm: LlmService,
    pub system_prompt: SystemPromptManager,
}

impl AgentRuntime {
    pub fn new(llm: LlmService) -> Self {
        let system_prompt = SystemPromptManager::new(
            "你是 CleanPilot 的系统级 Agent。你的唯一输出必须是 JSON 数组，且数组中的每一项都是一个任务对象。\n可用任务类型如下：\n1. {\"type\":\"assistant_reply\",\"content\":\"要展示给用户的文本\"}\n2. {\"type\":\"tool_call\",\"tool_name\":\"工具名\",\"payload\":\"传给工具的字符串参数\"}\n3. {\"type\":\"continue_reply\"}\n规则：\n- 只能输出合法 JSON，不能输出 markdown、解释、代码块。\n- 如果你只是要回复用户，就输出 assistant_reply 任务。\n- 如果你需要先调用工具，再继续基于结果思考，可以先输出 tool_call，再输出 continue_reply。\n- 如果暂时没有工具可用，就只输出 assistant_reply。\n- 默认优先输出简洁明确的 assistant_reply。"
                .to_string(),
        );

        Self {
            history: AgentHistory::new(system_prompt.build()),
            tasks: AgentTaskQueue::default(),
            llm,
            system_prompt,
        }
    }

    pub fn set_tool_prompt(&mut self, prompt: String) -> Result<(), String> {
        self.system_prompt.set_tool_prompt(prompt);
        self.history.set_system_prompt(self.system_prompt.build())
    }

    pub fn set_scene_prompt(&mut self, prompt: String) -> Result<(), String> {
        self.system_prompt.set_scene_prompt(prompt);
        self.history.set_system_prompt(self.system_prompt.build())
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
