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
                "   作用：写一条要加入历史记录的回复文本。它既可以是最终答复，也可以是工具调用前后的阶段性说明。",
                "2. {\"type\":\"tool_call\",\"tool_name\":\"工具名\",\"payload\":\"传给工具的字符串参数\"}",
                "   作用：当你需要调用某个工具获取额外信息时，输出这个任务。",
                "3. {\"type\":\"continue_reply\"}",
                "   作用：当你已经调用了工具，并希望系统基于工具结果继续思考时，输出这个任务。",
                "",
                "规则：",
                "- 只能输出合法 JSON，不能输出 markdown、解释、代码块。",
                "- 你可以在同一个 JSON 数组里同时输出多个任务，最好不要超过4条。",
                "- 如果你需要调用工具，可以输出 tool_call。",
                "- 如果你需要在工具执行完成后让系统继续基于结果思考，可以在 tool_call 后再输出 continue_reply。",
                "- 一般情况下，只要你输出了 tool_call，就应该同时输出 continue_reply。",
                "- 只有当工具返回结果后不需要你再做任何解释、总结或建议时，才可以省略 continue_reply。",
                "- 如果你已经拿到了足够信息，并且下一步可以直接结束本次任务，就不要输出 continue_reply。",
                "- 在输出新的 tool_call 之前，必须先检查历史里是否已经存在相同 tool_name 和相同 payload 的工具结果。",
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
