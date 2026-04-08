use std::path::Path;
use std::process::Command;

/// 当前“任务型存储”里建议保存的数据结构可以先按下面的语义约定：
///
/// 1. 顶层是一个列表。
/// 2. 列表中的每一个元素代表一条任务条目。
/// 3. 每个条目至少应包含：
///    - path: 文件或文件夹路径
///    - purpose: 该路径的用途、为什么保留它
///
/// 一个足够轻量、但已经有规范的 JSON 形态可以理解为：
/// {
///   "items": [
///     {
///       "path": "D:/example/file.txt",
///       "purpose": "这是清理分析后保留下来的配置文件",
///     }
///   ]
/// }
///
/// 当前先把这份注释放在这里，后面如果确认这个形态稳定，
/// 再正式提成 Rust 结构体会比较自然。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathCheckResult {
    Valid,
    Invalid,
}

pub fn check_path(path: &Path) -> PathCheckResult {
    if path.exists() {
        PathCheckResult::Valid
    } else {
        PathCheckResult::Invalid
    }
}

pub fn reveal_in_file_manager(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("路径不存在: {}", path.display()));
    }

    let status = if path.is_file() {
        Command::new("explorer")
            .arg(format!("/select,{}", path.display()))
            .status()
            .map_err(|e| format!("调用资源管理器失败: {}", e))?
    } else {
        Command::new("explorer")
            .arg(path)
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

    trash::delete(path)
        .map_err(|e| format!("移动到系统回收站失败: {}", e))
}
