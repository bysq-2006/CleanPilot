use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};
use crate::models::appstore::AppStore;

#[derive(Deserialize)]
struct WriteStorageBoxChecklistArgs {
    file_name: String,
    task_type: String,
    items: Vec<StorageBoxChecklistItem>,
}

#[derive(Deserialize)]
struct StorageBoxChecklistItem {
    path: String,
    purpose: String,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "write_storage_box_checklist",
        description: "在任务收尾阶段使用。把本次磁盘清理整理出的候选清单写入 storage box。清单中每个元素只能包含 path 和 purpose，可同时包含文件夹和单独文件。",
        usage: "arguments 传 JSON 字符串，例如 {\"file_name\":\"cleanup-checklist.json\",\"task_type\":\"disk_cleanup\",\"items\":[{\"path\":\"D:/Temp\",\"purpose\":\"临时文件目录，可进一步确认后清理\"},{\"path\":\"D:/a.log\",\"purpose\":\"单独日志文件，体积较大\"}] }。请只在分析收尾、准备给用户生成最终清单时调用。",
        handler: call,
    }
}

fn call(runtime: AgentRuntime, payload: String) -> ToolFuture {
    Box::pin(async move {
        let args: WriteStorageBoxChecklistArgs = serde_json::from_str(&payload)
            .map_err(|e| format!("任务失败：参数解析失败，请按要求传入 JSON。错误: {}", e))?;

        let file_name = args.file_name.trim();
        if file_name.is_empty() {
            return Err("任务失败：file_name 不能为空。".to_string());
        }

        let task_type = args.task_type.trim();
        if task_type.is_empty() {
            return Err("任务失败：task_type 不能为空。".to_string());
        }

        if args.items.is_empty() {
            return Err("任务失败：items 不能为空，至少要包含一条 path + purpose 清单项。".to_string());
        }

        for (index, item) in args.items.iter().enumerate() {
            if item.path.trim().is_empty() {
                return Err(format!("任务失败：第 {} 条清单项的 path 不能为空。", index + 1));
            }

            if item.purpose.trim().is_empty() {
                return Err(format!("任务失败：第 {} 条清单项的 purpose 不能为空。", index + 1));
            }
        }

        let normalized_items = args
            .items
            .into_iter()
            .map(|item| {
                serde_json::json!({
                    "path": item.path.trim(),
                    "purpose": item.purpose.trim(),
                })
            })
            .collect::<Vec<_>>();

        let content = serde_json::to_string_pretty(&serde_json::json!({
            "items": normalized_items,
        }))
        .map_err(|e| format!("任务失败：清单序列化失败。错误: {}", e))?;

        let app_handle: AppHandle = runtime
            .history
            .app_handle()
            .map_err(|e| format!("任务失败：无法获取应用句柄。错误: {}", e))?;
        let store = app_handle.state::<AppStore>();

        store
            .manager
            .storage_box
            .save_new_record(file_name.to_string(), content, task_type.to_string())
            .map_err(|e| format!("任务失败：写入 storage box 失败。错误: {}", e))?;

        Ok(format!(
            "已写入 storage box 清单\n文件名: {}\n条目数: {}",
            file_name,
            normalized_items.len()
        ))
    })
}
