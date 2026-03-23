use std::fs;
use std::path::Path;

use super::ToolDefinition;

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "list_directory",
        description: "查看指定路径下一层文件和文件夹的详细信息，并返回文本结果。",
        usage: "payload 直接传目录路径字符串，例如 D:/test 或 ./src",
        handler: call,
    }
}

fn call(payload: &str) -> Result<String, String> {
    let path = Path::new(payload.trim());

    if payload.trim().is_empty() {
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
}
