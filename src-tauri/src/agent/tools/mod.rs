mod list_directory;

const ENABLE_ALL_TOOLS: &str = "*";

#[derive(Clone)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub handler: fn(&str) -> Result<String, String>,
}

#[derive(Clone)]
pub struct ToolManager {
    tools: Vec<ToolDefinition>,
}

impl ToolManager {
    pub fn new(selection: &str) -> Self {
        let all_tools = vec![list_directory::register()];
        let selection = selection.trim();

        let tools = if selection.is_empty() {
            Vec::new()
        } else if selection == ENABLE_ALL_TOOLS {
            all_tools
        } else {
            let enabled_names = selection
                .split(',')
                .map(|item| item.trim())
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>();

            all_tools
                .into_iter()
                .filter(|tool| enabled_names.iter().any(|name| *name == tool.name))
                .collect()
        };

        Self { tools }
    }

    pub fn build_prompt(&self) -> String {
        let mut lines = vec!["可用工具如下：".to_string()];

        for tool in &self.tools {
            lines.push(format!("- 工具名：{}", tool.name));
            lines.push(format!("  作用：{}", tool.description));
            lines.push(format!("  用法：{}", tool.usage));
        }

        lines.join("\n")
    }

    pub fn call(&self, name: &str, payload: &str) -> Result<String, String> {
        let tool = self
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .ok_or_else(|| format!("未找到工具: {}", name))?;

        (tool.handler)(payload)
    }
}
