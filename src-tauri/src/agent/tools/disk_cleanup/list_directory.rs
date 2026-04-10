use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};
use crate::models::event_delegate::EventDelegate;

#[derive(Deserialize)]
struct ListDirectoryArgs {
    path: String,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "list_directory",
        description: "查看指定路径下一层文件和文件夹的详细信息，并返回文本结果。注意：该工具不会递归统计文件夹内部数据总大小，文件夹大小字段不是目录内容总占用。",
        usage: "arguments 传 JSON 字符串，例如 {\"path\":\"D:/test\"} 或 {\"path\":\"./src\"}。",
        handler: call,
    }
}

fn call(
    _runtime: AgentRuntime,
    _event_delegate: EventDelegate,
    payload: String,
) -> ToolFuture {
    Box::pin(async move {
        let args: ListDirectoryArgs = serde_json::from_str(&payload)
            .map_err(|e| format!("参数解析失败: {}", e))?;
        let path_str = args.path.trim();
        let path = Path::new(path_str);

        if path_str.is_empty() {
            return Err("目录路径不能为空".to_string());
        }

        if !path.exists() {
            return Err(format!("目录不存在: {}", path.display()));
        }

        if !path.is_dir() {
            return Err(format!("目标不是目录: {}", path.display()));
        }

        let mut lines = vec![format!("目录: {}", path.display())];
        let entries = fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let entry_path = entry.path();
            let metadata = entry
                .metadata()
                .map_err(|e| format!("读取文件元数据失败: {}", e))?;

            let kind = if metadata.is_dir() { "目录" } else { "文件" };
            let size = metadata.len();
            let name = entry.file_name().to_string_lossy().to_string();

            lines.push(format!(
                "- 名称: {} | 类型: {} | 大小: {} 字节 | 路径: {}",
                name,
                kind,
                size,
                entry_path.display()
            ));
        }

        Ok(lines.join("\n"))
    })
}

