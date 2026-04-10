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

pub fn move_to_trash(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("路径不存在: {}", path.display()));
    }

    if path.is_dir() {
        // 对于目录，只清理内部的直接子项，保留目录本身。
        // 每个子文件夹作为整体移入回收站，不递归拆散，保证速度和可恢复性。
        let entries = std::fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {} | 路径: {}", e, path.display()))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("遍历目录失败: {}", e))?;
            let child = entry.path();
            let clean = dunce::simplified(&child);
            trash::delete(clean).map_err(|e| {
                format!("移动到系统回收站失败: {} | 路径: {}", e, clean.display())
            })?;
        }

        Ok(())
    } else {
        let clean_path = dunce::simplified(path);
        trash::delete(clean_path)
            .map_err(|e| format!("移动到系统回收站失败: {} | 路径: {}", e, clean_path.display()))
    }
}
