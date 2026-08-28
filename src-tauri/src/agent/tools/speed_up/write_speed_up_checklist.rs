use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;
use crate::utils::process_live::is_protected_process;
use crate::utils::startup::StartupLocation;

#[derive(Debug, Serialize, Deserialize)]
struct SpeedUpProcessDraft {
    name: String,
    path: String,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpeedUpStartupDraft {
    name: String,
    location: String,
    command: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct WriteSpeedUpChecklistArgs {
    title: String,
    processes: Option<Vec<SpeedUpProcessDraft>>,
    startup_items: Option<Vec<SpeedUpStartupDraft>>,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "write_speed_up_checklist",
        description: "在任务收尾阶段使用。把本次「电脑变快」建议写入任务清单。processes 每项包含 name、path、reason；startup_items 每项包含 name、location、command、reason。location 必须原样使用启动项列表里的位置代码，例如 hkcu_run。不要写入系统关键或安全相关进程，也不要编造 PID。",
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "title": {"type": "string", "description": "本次建议的简短名称"},
                "processes": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "path": {"type": "string"},
                            "reason": {"type": "string"}
                        },
                        "required": ["name", "path", "reason"],
                        "additionalProperties": false
                    }
                },
                "startup_items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "location": {"type": "string", "description": "启动项位置代码，必须来自列表结果中的位置字段"},
                            "command": {"type": "string"},
                            "reason": {"type": "string"}
                        },
                        "required": ["name", "location", "command", "reason"],
                        "additionalProperties": false
                    }
                }
            },
            "required": ["title"],
            "additionalProperties": false
        }),
    }
}

pub async fn call(
    runtime: AgentRuntime,
    payload: String,
    cancellation_token: CancellationToken,
) -> Result<String, String> {
    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let args: WriteSpeedUpChecklistArgs = serde_json::from_str(&payload)
        .map_err(|e| format!("write_speed_up_checklist 参数解析失败: {}", e))?;

    if args.title.trim().is_empty() {
        return Err("write_speed_up_checklist title 不能为空".to_string());
    }

    let processes = args.processes.unwrap_or_default();
    let startup_items = args.startup_items.unwrap_or_default();

    if processes.is_empty() && startup_items.is_empty() {
        return Err("write_speed_up_checklist 至少需要一个进程或开机项".to_string());
    }

    for item in &processes {
        if cancellation_token.is_cancelled() {
            return Err("任务已取消".to_string());
        }
        if item.name.trim().is_empty() || item.path.trim().is_empty() || item.reason.trim().is_empty() {
            return Err("write_speed_up_checklist 进程项的 name、path、reason 都不能为空".to_string());
        }
        if is_protected_process(&item.name) {
            return Err(format!("不能把系统关键或安全相关进程写入清单: {}", item.name));
        }
    }

    for item in &startup_items {
        if cancellation_token.is_cancelled() {
            return Err("任务已取消".to_string());
        }
        if item.name.trim().is_empty() || item.location.trim().is_empty() || item.reason.trim().is_empty() {
            return Err("write_speed_up_checklist 开机项的 name、location、reason 都不能为空".to_string());
        }
        if StartupLocation::parse(&item.location).is_none() {
            return Err(format!("未知的开机项位置: {}", item.location));
        }
    }

    let message = serde_json::json!({
        "event": "write_speed_up_checklist",
        "title": args.title.trim(),
        "processes": processes,
        "startup_items": startup_items,
    });

    tokio::select! {
        _ = cancellation_token.cancelled() => return Err("任务已取消".to_string()),
        result = runtime.event_delegate.request(message.to_string()) => {
            result.map_err(|e| format!("写入加速建议失败: {}", e))?;
        }
    }

    Ok("加速建议已保存到任务".to_string())
}
