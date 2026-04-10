use serde::{Deserialize, Serialize};

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};
use crate::models::event_delegate::EventDelegate;

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
        usage: "传入 title（本次清理的简短名称，如「下载目录清理」）和 content 数组，例如 {\"title\":\"下载目录清理\",\"content\":[{\"path\":\"C:/Users/19045/Downloads/a.zip\",\"purpose\":\"下载目录中的大文件，可确认后清理\"}]}。不要生成 file_name、saved_at、task_type，这些字段由程序内部自动补齐。",
        handler: call,
    }
}

fn call(
    runtime: AgentRuntime,
    event_delegate: EventDelegate,
    payload: String,
) -> ToolFuture {
    Box::pin(async move {
        let _ = runtime;

        let args: WriteStorageBoxChecklistArgs = serde_json::from_str(&payload)
            .map_err(|e| format!("write_storage_box_checklist 参数解析失败: {}", e))?;

        if args.title.trim().is_empty() {
            return Err("write_storage_box_checklist title 不能为空".to_string());
        }

        if args.content.is_empty() {
            return Err("write_storage_box_checklist content 不能为空".to_string());
        }

        for item in &args.content {
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

        event_delegate
            .sender
            .send(message.to_string())
            .await
            .map_err(|e| format!("发送 storage box 委托消息失败: {}", e))?;

        Ok("storage box 清单写入请求已提交".to_string())
    })
}
