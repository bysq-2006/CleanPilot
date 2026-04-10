use std::path::Path;
use std::process::Command;

pub fn reveal_in_file_manager(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("路径不存在: {}", path.display()));
    }

    let status = if path.is_file() {
        let canonical_path = path
            .canonicalize()
            .map_err(|e| format!("规范化文件路径失败: {}", e))?;

        Command::new("explorer")
            .arg("/select,")
            .arg(&canonical_path)
            .status()
            .map_err(|e| format!("调用资源管理器失败: {}", e))?
    } else {
        let canonical_path = path
            .canonicalize()
            .map_err(|e| format!("规范化目录路径失败: {}", e))?;

        Command::new("explorer")
            .arg(&canonical_path)
            .status()
            .map_err(|e| format!("调用资源管理器失败: {}", e))?
    };

    if !status.success() {
        return Err(format!("资源管理器返回非成功状态: {}", status));
    }

    Ok(())
}

