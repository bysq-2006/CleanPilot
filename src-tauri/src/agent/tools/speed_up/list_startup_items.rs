use tokio_util::sync::CancellationToken;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;
use crate::utils::startup::collect_startup_items;

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "list_startup_items",
        description: "列出开机启动项，包括注册表 Run 项和启动文件夹中的程序，并尽量标注是否已禁用。每条都包含位置代码（如 hkcu_run），后续写入任务清单时必须原样使用该位置。不包含计划任务。",
        parameters: serde_json::json!({"type":"object","properties":{},"additionalProperties":false}),
    }
}

pub async fn call(
    _runtime: AgentRuntime,
    _payload: String,
    cancellation_token: CancellationToken,
) -> Result<String, String> {
    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let mut items = tokio::task::spawn_blocking(collect_startup_items)
        .await
        .map_err(|e| format!("读取开机启动项失败: {}", e))?;

    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    items.sort_by(|left, right| {
        right
            .enabled
            .cmp(&left.enabled)
            .then_with(|| left.name.to_ascii_lowercase().cmp(&right.name.to_ascii_lowercase()))
    });

    let mut lines = vec![format!("启动项数量: {}", items.len())];
    if items.is_empty() {
        lines.push("未找到开机启动项。".to_string());
        return Ok(lines.join("\n"));
    }

    for item in items {
        if cancellation_token.is_cancelled() {
            return Err("任务已取消".to_string());
        }

        lines.push(format!(
            "- 名称: {} | 状态: {} | 位置: {} | 来源: {} | 命令: {}",
            item.name,
            if item.enabled { "已启用" } else { "已禁用" },
            item.location.as_str(),
            item.location.source_label(),
            item.command
        ));
    }

    Ok(lines.join("\n"))
}
