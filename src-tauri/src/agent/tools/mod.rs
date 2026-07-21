mod disk_cleanup;
mod utility;

use crate::agent::runtime::AgentRuntime;
use crate::models::event_delegate::EventDelegate;
use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

pub type ToolFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
pub type ToolHandler = fn(AgentRuntime, EventDelegate, String) -> ToolFuture;

const ENABLE_ALL_TOOLS: &str = "*";

#[derive(Clone)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters: Value,
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
            disk_cleanup::write_storage_box_checklist::register(),
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

    pub fn api_tools(&self) -> Vec<ChatCompletionTool> {
        self.tools
            .iter()
            .map(|tool| ChatCompletionTool {
                r#type: ChatCompletionToolType::Function,
                function: FunctionObject {
                    name: tool.name.to_string(),
                    description: Some(tool.description.to_string()),
                    parameters: Some(tool.parameters.clone()),
                    strict: None,
                },
            })
            .collect()
    }

    pub async fn call(&self, runtime: &AgentRuntime, name: &str, payload: &str) -> Result<String, String> {
        let tool = self
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .ok_or_else(|| format!("未找到工具: {}", name))?;

        (tool.handler)(runtime.clone(), runtime.event_delegate.clone(), payload.to_string()).await
    }
}
