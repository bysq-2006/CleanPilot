use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};

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
        Ok("write_storage_box_checklist 工具尚未实现".to_string())
    })
}
