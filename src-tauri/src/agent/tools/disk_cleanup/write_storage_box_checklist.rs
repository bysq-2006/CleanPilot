use serde::Deserialize;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};
use crate::models::event_delegate::EventDelegate;

#[derive(Debug, Deserialize)]
struct StorageBoxChecklistItem {
    path: String,
    purpose: String,
}

#[derive(Debug, Deserialize)]
struct WriteStorageBoxChecklistArgs {
    content: Vec<StorageBoxChecklistItem>,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "write_storage_box_checklist",
        description: "在任务收尾阶段使用。把本次磁盘清理整理出的候选清单写入 storage box。清单中每个元素只能包含 path 和 purpose，可同时包含文件夹和单独文件。",
        usage: "arguments 只传 JSON 字符串中的 content 数组，例如 {\"content\":[{\"path\":\"C:/Users/19045/Downloads/a.zip\",\"purpose\":\"下载目录中的大文件，可确认后清理\"},{\"path\":\"C:/Users/19045/Desktop/old.log\",\"purpose\":\"旧日志文件，可删除\"}] }。不要生成 file_name、saved_at、task_type，这些字段由程序内部自动补齐。",
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
            "content": args
                .content
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "path": item.path,
                        "purpose": item.purpose,
                    })
                })
                .collect::<Vec<_>>(),
        });

        event_delegate
            .sender
            .send(message.to_string())
            .await
            .map_err(|e| format!("发送 storage box 委托消息失败: {}", e))?;

        Ok("storage box 清单写入请求已提交".to_string())
    })
}
