mod disk_cleanup;
mod utility;

use crate::agent::runtime::AgentRuntime;
use std::future::Future;
use std::pin::Pin;

pub type ToolFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
pub type ToolHandler = fn(AgentRuntime, String) -> ToolFuture;

const ENABLE_ALL_TOOLS: &str = "*";

#[derive(Clone)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub handler: ToolHandler,
}

#[derive(Clone)]
pub struct ToolManager {
    tools: Vec<ToolDefinition>,
}

impl ToolManager {
    /// 可以选择启用全部工具（传入 "*"），也可以启用部分工具（传入逗号分隔的工具名列表，例如 "list_directory,disk_info"）
    /// 或者不启用任何工具（传入空字符串）。
    pub fn new(selection: &str) -> Self {
        let all_tools = vec![
            disk_cleanup::list_directory::register(),
            disk_cleanup::disk_info::register(),
            disk_cleanup::find_large_entries::register(),
            utility::file_read::register(),
            utility::http_request::register(),
        ];
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

    pub async fn call(&self, runtime: &AgentRuntime, name: &str, payload: &str) -> Result<String, String> {
        let tool = self
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .ok_or_else(|| format!("未找到工具: {}", name))?;

        (tool.handler)(runtime.clone(), payload.to_string()).await
    }
}
