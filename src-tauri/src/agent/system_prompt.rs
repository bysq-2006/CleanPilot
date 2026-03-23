#[derive(Clone)]
pub struct SystemPromptManager {
    core_prompt: String,
    tool_prompt: String,
    scene_prompt: String,
}

impl SystemPromptManager {
    pub fn new() -> Self {
        Self {
            core_prompt: [
                "你是 CleanPilot 的系统级 Agent。",
                "你的唯一输出必须是 JSON 数组，且数组中的每一项都是一个任务对象。",
                "",
                "可用任务类型如下：",
                "1. {\"type\":\"assistant_reply\",\"content\":\"要展示给用户的文本\"}",
                "2. {\"type\":\"tool_call\",\"tool_name\":\"工具名\",\"payload\":\"传给工具的字符串参数\"}",
                "3. {\"type\":\"continue_reply\"}",
                "",
                "规则：",
                "- 只能输出合法 JSON，不能输出 markdown、解释、代码块。",
                "- 如果你只是要回复用户，就输出 assistant_reply 任务。",
                "- 如果你需要先调用工具，再继续基于结果思考，可以先输出 tool_call，再输出 continue_reply。",
                "- 如果暂时没有工具可用，就只输出 assistant_reply。",
                "- 默认优先输出简洁明确的 assistant_reply。",
            ]
            .join("\n"),
            tool_prompt: String::new(),
            scene_prompt: String::new(),
        }
    }

    pub fn build(&self) -> String {
        let mut parts = vec![self.core_prompt.clone()];

        if !self.tool_prompt.is_empty() {
            parts.push(self.tool_prompt.clone());
        }

        if !self.scene_prompt.is_empty() {
            parts.push(self.scene_prompt.clone());
        }

        parts.join("\n\n")
    }

    pub fn set_tool_prompt(&mut self, prompt: String) {
        self.tool_prompt = prompt;
    }

    pub fn set_scene_prompt(&mut self, prompt: String) {
        self.scene_prompt = prompt;
    }
}
