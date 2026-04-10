use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};
use crate::models::event_delegate::EventDelegate;

#[derive(Deserialize)]
struct FileReadArgs {
    path: String,
    max_chars: Option<usize>,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "file_read",
        description: "读取指定文件的文本内容，可选限制返回字符数，适合查看配置、日志或代码片段。",
        usage: "arguments 传 JSON 字符串，例如 {\"path\":\"./README.md\"} 或 {\"path\":\"C:/temp/test.log\",\"max_chars\":4000}。",
        handler: call,
    }
}

fn call(
    _runtime: AgentRuntime,
    _event_delegate: EventDelegate,
    payload: String,
) -> ToolFuture {
    Box::pin(async move {
        let args: FileReadArgs = serde_json::from_str(&payload)
            .map_err(|e| format!("文件读取失败\n参数解析失败: {}", e))?;
        let path_str = args.path.trim();

        if path_str.is_empty() {
            return Err("文件读取失败\n文件路径不能为空".to_string());
        }

        let path = Path::new(path_str);
        if !path.exists() {
            return Err(format!("文件读取失败\n文件不存在: {}", path.display()));
        }

        if !path.is_file() {
            return Err(format!("文件读取失败\n目标不是文件: {}", path.display()));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("文件读取失败\n读取文件内容失败: {}", e))?;
        let char_count = content.chars().count();
        let max_chars = args.max_chars.unwrap_or(8000).clamp(1, 50000);

        let (display_content, truncated) = if char_count > max_chars {
            (
                content.chars().take(max_chars).collect::<String>(),
                true,
            )
        } else {
            (content, false)
        };

        Ok(format!(
            "文件路径: {}\n字符数: {}\n返回字符数: {}\n是否截断: {}\n文件内容:\n{}",
            path.display(),
            char_count,
            display_content.chars().count(),
            if truncated { "是" } else { "否" },
            display_content
        ))
    })
}
