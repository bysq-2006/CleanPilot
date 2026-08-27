mod disk_cleanup;
mod speed_up;
mod utility;

use crate::agent::runtime::AgentRuntime;
use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;
use tokio_util::sync::CancellationToken;

const ENABLE_ALL_TOOLS: &str = "*";

#[derive(Clone)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters: Value,
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
            speed_up::system_perf::register(),
            speed_up::list_processes::register(),
            speed_up::list_startup_items::register(),
            speed_up::write_speed_up_checklist::register(),
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

    pub async fn call(&self, runtime: &AgentRuntime, name: &str, payload: &str, cancellation_token: &CancellationToken) -> Result<String, String> {
        if cancellation_token.is_cancelled() {
            return Err("任务已取消".to_string());
        }
        if !self.tools.iter().any(|tool| tool.name == name) {
            return Err(format!("未找到工具: {}", name));
        }

        let runtime = runtime.clone();
        let payload = payload.to_string();

        match name {
            "list_directory" => disk_cleanup::list_directory::call(runtime, payload, cancellation_token.clone()).await,
            "get_disk_info" => disk_cleanup::disk_info::call(runtime, payload, cancellation_token.clone()).await,
            "find_large_entries" => disk_cleanup::find_large_entries::call(runtime, payload, cancellation_token.clone()).await,
            "write_storage_box_checklist" => {
                disk_cleanup::write_storage_box_checklist::call(runtime, payload, cancellation_token.clone()).await
            }
            "file_read" => utility::file_read::call(runtime, payload, cancellation_token.clone()).await,
            "http_request" => utility::http_request::call(runtime, payload, cancellation_token.clone()).await,
            "get_system_perf" => speed_up::system_perf::call(runtime, payload, cancellation_token.clone()).await,
            "list_processes" => speed_up::list_processes::call(runtime, payload, cancellation_token.clone()).await,
            "list_startup_items" => {
                speed_up::list_startup_items::call(runtime, payload, cancellation_token.clone()).await
            }
            "write_speed_up_checklist" => {
                speed_up::write_speed_up_checklist::call(runtime, payload, cancellation_token.clone()).await
            }
            _ => Err(format!("未找到工具: {}", name)),
        }
    }
}
