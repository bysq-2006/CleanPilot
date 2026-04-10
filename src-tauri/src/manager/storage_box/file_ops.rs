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
        return move_directory_contents_to_trash(path);
    }

    trash::delete(path)
        .map_err(|e| format!("移动到系统回收站失败: {}", e))
}

fn move_directory_contents_to_trash(dir: &Path) -> Result<(), String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("读取目录失败，无法移入回收站: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("遍历目录失败，无法移入回收站: {}", e))?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            move_directory_contents_to_trash(&entry_path)?;
        }
        else {
            trash::delete(&entry_path).map_err(|e| {
                format!(
                    "移动目录内文件到系统回收站失败: {} | 路径: {}",
                    e,
                    entry_path.display()
                )
            })?;
        }
    }

    trash::delete(dir)
        .map_err(|e| format!("移动目录到系统回收站失败: {} | 路径: {}", e, dir.display()))
}
