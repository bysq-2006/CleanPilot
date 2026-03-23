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
                "你的唯一输出必须是一条合法 JSON 对象，表示一条 assistant 消息。",
                "",
                "输出格式如下：",
                "{\"type\":\"assistant\",\"content\":\"要展示给用户的文本\",\"tool_calls\":[...]}",
                "其中 content 可以为空字符串或省略，tool_calls 可以省略。",
                "",
                "规则：",
                "- 只能输出合法 JSON，不能输出 markdown、解释、代码块。",
                "- type 必须固定为 assistant。",
                "- 只要结论依赖外部信息、目录内容、文件状态或工具结果，就必须先调用工具，不能直接猜测。",
                "- 如果你在文本里提到‘接下来查看某目录’、‘我来检查’、‘我来分析某路径’，那一轮必须同时给出对应 tool_calls，不能只写说明文字。",
                "- 如果你需要调用工具，请在 tool_calls 中输出标准 OpenAI 风格的 function 调用结构。",
                "- 如果你不需要调用工具，就不要输出 tool_calls。",
                "- 如果你已经拿到了足够信息，请直接在 content 中给出最终答复。",
                "- 在输出新的 tool_calls 之前，必须先检查历史里是否已经存在相同工具和相同参数的结果。",
                "- 如果相同工具结果已经存在，你不能再次调用它，必须改为总结、结束，或选择新的目标。",
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
