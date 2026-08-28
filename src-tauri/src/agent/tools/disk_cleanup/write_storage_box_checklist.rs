use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;

#[derive(Debug, Serialize, Deserialize)]
struct StorageBoxChecklistItem {
    path: String,
    purpose: String,
}

#[derive(Debug, Deserialize)]
struct WriteStorageBoxChecklistArgs {
    title: String,
    content: Vec<StorageBoxChecklistItem>,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "write_storage_box_checklist",
        description: "在任务收尾阶段使用。把本次磁盘清理整理出的候选清单写入 storage box。清单中每个元素只能包含 path 和 purpose，可同时包含文件夹和单独文件。",
        parameters: serde_json::json!({"type":"object","properties":{"title":{"type":"string","description":"本次清理的简短名称"},"content":{"type":"array","items":{"type":"object","properties":{"path":{"type":"string"},"purpose":{"type":"string"}},"required":["path","purpose"],"additionalProperties":false}}},"required":["title","content"],"additionalProperties":false}),
    }
}

pub async fn call(
    runtime: AgentRuntime,
    payload: String,
    cancellation_token: CancellationToken,
) -> Result<String, String> {
        if cancellation_token.is_cancelled() { return Err("任务已取消".to_string()); }
        let args: WriteStorageBoxChecklistArgs = serde_json::from_str(&payload)
            .map_err(|e| format!("write_storage_box_checklist 参数解析失败: {}", e))?;

        if args.title.trim().is_empty() {
            return Err("write_storage_box_checklist title 不能为空".to_string());
        }

        if args.content.is_empty() {
            return Err("write_storage_box_checklist content 不能为空".to_string());
        }

        for item in &args.content {
            if cancellation_token.is_cancelled() { return Err("任务已取消".to_string()); }
            if item.path.trim().is_empty() {
                return Err("write_storage_box_checklist 存在空 path".to_string());
            }

            if item.purpose.trim().is_empty() {
                return Err("write_storage_box_checklist 存在空 purpose".to_string());
            }
        }

        let message = serde_json::json!({
            "event": "write_storage_box_checklist",
            "title": args.title.trim(),
            "content": args.content,
        });

        tokio::select! {
            _ = cancellation_token.cancelled() => return Err("任务已取消".to_string()),
            result = runtime.event_delegate.request(message.to_string()) => {
                result.map_err(|e| format!("写入 storage box 失败: {}", e))?;
            }
        }

    Ok("清理清单已保存到任务".to_string())
}
