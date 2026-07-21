#[derive(Clone)]
pub struct SystemPromptManager {
    core_prompt: String,
    scene_prompt: String,
}

impl SystemPromptManager {
    pub fn new() -> Self {
        Self {
            core_prompt: [
                "你是 CleanPilot 的系统级 Agent。",
                "规则：",
                "- 只要结论依赖外部信息、目录内容、文件状态或工具结果，就必须先调用工具，不能直接猜测。",
                "- 如果你已经拿到了足够信息，请直接给出最终答复。",
                "- 调用工具前，必须先检查历史里是否已经存在相同工具和相同参数的结果。",
                "- 如果相同工具结果已经存在，你不能再次调用它，必须改为总结、结束，或选择新的目标。",
            ]
            .join("\n"),
            scene_prompt: String::new(),
        }
    }

    pub fn build(&self) -> String {
        let mut parts = vec![self.core_prompt.clone()];

        if !self.scene_prompt.is_empty() {
            parts.push(self.scene_prompt.clone());
        }

        parts.join("\n\n")
    }

    pub fn set_scene_prompt(&mut self, prompt: String) {
        self.scene_prompt = prompt;
    }
}
