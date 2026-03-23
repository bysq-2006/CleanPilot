#[derive(Clone)]
pub struct SystemPromptManager {
    core_prompt: String,
    tool_prompt: String,
    scene_prompt: String,
}

impl SystemPromptManager {
    pub fn new(core_prompt: String) -> Self {
        Self {
            core_prompt,
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
