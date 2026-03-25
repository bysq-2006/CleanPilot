use std::path::Path;

use serde::Deserialize;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;

#[derive(Deserialize)]
struct DiskInfoArgs {
    path: String,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "get_disk_info",
        description: "获取指定路径所在磁盘的总容量、可用空间与已用空间。",
        usage: "arguments 传 JSON 字符串，例如 {\"path\":\"C:/\"} 或 {\"path\":\"./src\"}",
        handler: call,
    }
}

fn call(_runtime: &AgentRuntime, payload: &str) -> Result<String, String> {
    let args: DiskInfoArgs = serde_json::from_str(payload)
        .map_err(|e| format!("参数解析失败: {}", e))?;
    let path_str = args.path.trim();

    if path_str.is_empty() {
        return Err("磁盘路径不能为空".to_string());
    }

    let path = Path::new(path_str);
    let canonical = path
        .canonicalize()
        .map_err(|e| format!("路径解析失败: {}", e))?;
    let normalized_canonical = normalize_windows_path(&canonical.display().to_string());

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disk = disks
        .into_iter()
        .find(|disk| {
            let mount_point = normalize_windows_path(&disk.mount_point().display().to_string());
            normalized_canonical.starts_with(&mount_point)
        })
        .ok_or_else(|| format!("未找到路径所在磁盘: {}", canonical.display()))?;

    let total_space = disk.total_space();
    let available_space = disk.available_space();
    let used_space = total_space.saturating_sub(available_space);

    Ok(format!(
        "路径: {}\n挂载点: {}\n文件系统: {}\n总容量: {} 字节\n可用空间: {} 字节\n已用空间: {} 字节",
        canonical.display(),
        disk.mount_point().display(),
        disk.file_system().to_string_lossy(),
        total_space,
        available_space,
        used_space
    ))
}

fn normalize_windows_path(path: &str) -> String {
    path.trim_start_matches(r"\\?\")
        .replace('/', r"\")
        .to_ascii_lowercase()
}
