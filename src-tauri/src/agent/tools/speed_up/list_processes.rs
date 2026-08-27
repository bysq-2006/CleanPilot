use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;

const DEFAULT_LIMIT: usize = 20;
const MAX_LIMIT: usize = 50;

#[derive(Deserialize, Default)]
struct ListProcessesArgs {
    sort_by: Option<String>,
    limit: Option<u32>,
}

struct ProcessRow {
    name: String,
    pid: u32,
    cpu: f32,
    memory: u64,
    disk_read: u64,
    disk_write: u64,
    category: &'static str,
    path: String,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "list_processes",
        description: "列出当前占用较高的进程，可按 CPU、内存或磁盘排序。返回进程名、PID、占用和可执行文件路径。适合找出正在拖慢电脑的程序。默认按 CPU 占用返回前 20 个。CPU 占用为相对单核百分比，多核上可能超过 100%。",
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "sort_by": {"type": "string", "enum": ["cpu", "memory", "disk"], "description": "排序方式，默认 cpu"},
                "limit": {"type": "integer", "description": "返回条数，默认 20，最大 50"}
            },
            "additionalProperties": false
        }),
    }
}

pub async fn call(
    _runtime: AgentRuntime,
    payload: String,
    cancellation_token: CancellationToken,
) -> Result<String, String> {
    let args: ListProcessesArgs = super::parse_optional_args(&payload)?;
    let sort_by = args
        .sort_by
        .as_deref()
        .unwrap_or("cpu")
        .trim()
        .to_ascii_lowercase();
    if !matches!(sort_by.as_str(), "cpu" | "memory" | "disk") {
        return Err("sort_by 只能是 cpu、memory 或 disk".to_string());
    }

    let limit = args
        .limit
        .map(|value| value as usize)
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_LIMIT)
        .min(MAX_LIMIT);

    let sys = super::refreshed_system(&cancellation_token).await?;
    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let mut rows = sys
        .processes()
        .values()
        .filter_map(|process| {
            let name = process.name().trim();
            if name.is_empty() {
                return None;
            }

            let disk = process.disk_usage();
            Some(ProcessRow {
                name: name.to_string(),
                pid: process.pid().as_u32(),
                cpu: process.cpu_usage(),
                memory: process.memory(),
                disk_read: disk.read_bytes,
                disk_write: disk.written_bytes,
                category: process_category(name),
                path: process
                    .exe()
                    .map(|path| super::normalize_windows_path(&path.display().to_string()))
                    .unwrap_or_else(|| "未知".to_string()),
            })
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| match sort_by.as_str() {
        "memory" => right.memory.cmp(&left.memory).then_with(|| left.name.cmp(&right.name)),
        "disk" => {
            let left_disk = left.disk_read.saturating_add(left.disk_write);
            let right_disk = right.disk_read.saturating_add(right.disk_write);
            right_disk.cmp(&left_disk).then_with(|| left.name.cmp(&right.name))
        }
        _ => right
            .cpu
            .partial_cmp(&left.cpu)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.name.cmp(&right.name)),
    });
    rows.truncate(limit);

    let mut lines = vec![
        format!("排序: {}", sort_by),
        format!("显示数量: {}", rows.len()),
        "CPU 占用为相对单核百分比，多核上可能超过 100%。磁盘读写为两次采样之间的增量。".to_string(),
    ];

    if rows.is_empty() {
        lines.push("未找到可展示的进程。".to_string());
        return Ok(lines.join("\n"));
    }

    for row in rows {
        if cancellation_token.is_cancelled() {
            return Err("任务已取消".to_string());
        }

        lines.push(format!(
            "- 名称: {} | PID: {} | CPU: {:.1}% | 内存: {} 字节 | 磁盘读取: {} 字节 | 磁盘写入: {} 字节 | 类别: {} | 路径: {}",
            row.name,
            row.pid,
            row.cpu,
            row.memory,
            row.disk_read,
            row.disk_write,
            row.category,
            row.path
        ));
    }

    Ok(lines.join("\n"))
}

fn process_category(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    let stem = lower.trim_end_matches(".exe");

    const CRITICAL: &[&str] = &[
        "system",
        "registry",
        "smss",
        "csrss",
        "wininit",
        "services",
        "lsass",
        "svchost",
        "winlogon",
        "dwm",
        "secure system",
        "memory compression",
        "idle",
        "system idle process",
        "fontdrvhost",
        "lsaiso",
        "conhost",
        "sihost",
        "runtimebroker",
        "searchhost",
        "shellexperiencehost",
        "textinputhost",
        "ctfmon",
        "taskhostw",
        "dllhost",
        "explorer",
    ];
    const SECURITY: &[&str] = &[
        "msmpeng",
        "nissrv",
        "securityhealthservice",
        "securityhealthsystray",
        "smartscreen",
        "mpdefendercoreservice",
        "securityhealthhost",
    ];

    if CRITICAL.iter().any(|item| stem == *item) {
        "系统关键"
    } else if SECURITY.iter().any(|item| stem == *item) {
        "安全软件"
    } else {
        "普通"
    }
}
